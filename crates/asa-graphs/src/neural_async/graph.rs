use std::{
    sync::{ Arc, RwLock },
    cmp::Ordering::*,
    marker::PhantomData,
    fmt::{ Display, Formatter, Result as FmtResult }
};

use anyhow::Result;

use witchnet_common::{ 
    sensor::SensorData,
    neuron::NeuronAsync,
    data::{ DataCategory, DataType, DataDeductor, DataTypeValue },
};

use super::{
    element::Element,
    node::Node
};

#[derive(Clone)]
pub struct ASAGraph<Key, const ORDER: usize = 25>
where Key: SensorData + Sync + Send, [(); ORDER + 1]: {
    pub id: u32,
    pub(crate) root: Arc<RwLock<Node<Key, ORDER>>>,
    pub element_min: Option<Arc<RwLock<Element<Key, ORDER>>>>,
    pub element_max: Option<Arc<RwLock<Element<Key, ORDER>>>>,
    pub elements_counter: u32,
    pub key_min: Option<Key>,
    pub key_max: Option<Key>,
    pub(crate) data_type: PhantomData<Key>
}

impl<Key, const ORDER: usize> ASAGraph<Key, ORDER> 
where 
    Key: SensorData + Sync + Send, 
    [(); ORDER + 1]:, 
    PhantomData<Key>: DataDeductor,
    DataTypeValue: From<Key>
{
    pub fn new(id: u32) -> ASAGraph<Key, ORDER> {
        if ORDER < 3 { panic!("Graph order must be >= 3") }
        ASAGraph {
            id,
            root: Arc::new(RwLock::new(Node::<Key, ORDER>::new(true, None))),
            element_min: None,
            element_max: None,
            elements_counter: 0,
            key_min: None,
            key_max: None,
            data_type: PhantomData
        }
    }

    pub fn new_arc(id: u32) -> Arc<RwLock<ASAGraph<Key, ORDER>>> {
        Arc::new(RwLock::new(ASAGraph::new(id)))
    }

    pub fn new_box(id: u32) -> Box<ASAGraph<Key, ORDER>> {
        Box::new(ASAGraph::new(id))
    }

    pub fn new_from_vec(id: u32, data: &[Key]) -> Self {
        let mut graph = Self::new(id);
        for point in data { graph.insert(point); }
        graph
    }

    pub fn new_rc_from_vec(id: u32, data: &[Key]) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new_from_vec(id, data)))
    }

    pub fn new_box_from_vec(id: u32, data: &[Key]) -> Box<Self> {
        Box::new(Self::new_from_vec(id, data))
    }
    
    pub fn id(&self) -> u32 { self.id }

    pub fn data_type(&self) -> DataType { self.data_type.data_type() }

    pub fn data_category(&self) -> DataCategory { self.data_type.data_category() }

    pub fn search(&self, key: &Key) -> Option<Arc<RwLock<Element<Key, ORDER>>>> {
        let node = &self.root;
        
        let (key_min, key_max) = self.extreme_keys()?;

        if key.distance(key_max) > key.distance(key_min) {
            return Self::search_left(key, node.clone())
        } else {
            return Self::search_right(key, node.clone())
        }
    }

    fn search_left<'a, 'b>(
        key: &'a Key, mut node_ptr: Arc<RwLock<Node<Key, ORDER>>>
    ) -> Option<Arc<RwLock<Element<Key, ORDER>>>> {
        loop {
            let next_node_ptr;
            {
                let node = node_ptr.read().unwrap();
                let mut index = 0;
                let mut current_key = node.keys[index].as_ref().unwrap();
                
                while index < node.size && key.partial_compare(current_key) == Some(Greater) {
                    index += 1;
                    if index < node.size {
                        current_key = node.keys[index].as_ref().unwrap();
                    }
                }
    
                if index < node.size && key.equals(current_key) {
                    let element = node.elements[index].as_ref().unwrap().clone();
                    return Some(element)
                } else if node.is_leaf {
                    return None
                }

                next_node_ptr = node.children[index].as_ref().unwrap().clone();
            }
            
            node_ptr = next_node_ptr;
        }
    }

    fn search_right<'a, 'b>(
        key: &'a Key, mut node_ptr: Arc<RwLock<Node<Key, ORDER>>>
    ) -> Option<Arc<RwLock<Element<Key, ORDER>>>> {
        loop {
            let next_node_ptr;
            {
                let node = node_ptr.read().unwrap();
                let mut index = node.size - 1;

                let mut current_key = node.keys[index].as_ref().unwrap();
                
                while index > 0 && key.partial_compare(current_key) == Some(Less) {
                    index -= 1;
                    current_key = node.keys[index].as_ref().unwrap();
                }

                if key.equals(current_key) {
                    let element = node.elements[index].as_ref().unwrap().clone();
                    return Some(element)
                } else if node.is_leaf {
                    return None
                } else if key.partial_compare(current_key) == Some(Greater) {
                    index += 1;
                }

                next_node_ptr = node.children[index].as_ref().unwrap().clone();
            }
            node_ptr = next_node_ptr;
        }
    }

    pub fn insert(&mut self, key: &Key) -> Arc<RwLock<Element<Key, ORDER>>> {
        let mut node = self.root.clone();

        if node.read().unwrap().size == 0 { return self.insert_first_element(&node, key) }

        if node.read().unwrap().size == Node::<Key, ORDER>::MAX_KEYS { node = self.split_root(); }

        let (key_min, key_max) = self.extreme_keys().unwrap_or_else(|| {
            panic!("element_min / element_min must not be nullptr")
        });

        loop {
            let node_insert_result = if key.distance(key_max) > key.distance(key_min) {
                node.read().unwrap().insert_existing_key(key, true)
            } else {
                node.read().unwrap().insert_existing_key(key, false)
            };
            if let Some(el) = node_insert_result.0 { return el }
            let mut index = node_insert_result.1;
    
            if node.read().unwrap().is_leaf {
                let element_id = self.elements_counter + 1;
                let element = Node::insert_key_leaf(&node, key, element_id, self.id, self.range());
                self.elements_counter += 1;
                self.set_extrema(&element);
                return element
            } else {
                let child_size = node.read().unwrap().children[index].as_ref().unwrap().read().unwrap().size;
                if child_size == Node::<Key, ORDER>::MAX_KEYS {
                    Node::split_child(&node, index);
                    if key.partial_compare(
                        &node.read().unwrap().elements[index].as_ref().unwrap().read().unwrap().key
                    ) == Some(Greater) {
                        index += 1 
                    } else if key.equals(node.read().unwrap().keys[index].as_ref().unwrap()) {
                        return node.read().unwrap().elements[index].as_ref().unwrap().clone()
                    }
                }
                let new_node = node.read().unwrap().children[index].as_ref().unwrap().clone();
                node = new_node.clone();
            }
        }
    }

    pub fn range(&self) -> f32 { 
        if self.key_min.is_none() || self.key_max.is_none() { return f32::NAN }
        let ret = self.key_min.as_ref().unwrap().distance(self.key_max.as_ref().unwrap()) as f32;
        if ret == 0.0f32 { 1.0f32 } else { ret }
     }

    pub fn print_graph(&self) {
        let mut height = 0;
        let mut node = self.root.clone();
        let mut queue: Vec<Vec<Arc<RwLock<Node<Key, ORDER>>>>> = vec![vec![]];
        queue[0].push(node.clone());

        loop {
            queue.push(vec![]);
            for i in 0..(queue[height].len()) {
                node = queue[height][i].clone();
                let node_size = node.read().unwrap().size;
                print!("||");
                for j in 0..(node_size) {
                    let element = node.read().unwrap().elements[j].as_ref().unwrap().clone();
                    print!("{}:{}|", &element.read().unwrap().key, element.read().unwrap().counter);
                    if !node.read().unwrap().is_leaf {
                        queue[height + 1].push(node.read().unwrap().children[j].as_ref().unwrap().clone());
                    }
                }
                if !node.read().unwrap().is_leaf {
                    queue[height + 1].push(node.read().unwrap().children[node_size].as_ref().unwrap().clone());
                }
                print!("| ");
            }
            if queue.last().unwrap().len() > 0 {
                height += 1;
                println!("");
            } else {
                println!("");
                return
            }
        }
    }

    fn extreme_keys<'a>(&'a self) -> Option<(&'a Key, &'a Key)> {
        if self.key_min.is_none() || self.key_max.is_none() { return None }
        let key_min =  self.key_min.as_ref().unwrap();
        let key_max =  self.key_max.as_ref().unwrap();
        Some((key_min, key_max))
    }

    fn insert_first_element(
        &mut self, node: &Arc<RwLock<Node<Key, ORDER>>>,  key: &Key
    ) -> Arc<RwLock<Element<Key, ORDER>>> {
        let element_pointer = Element::<Key, ORDER>::new(key, 1, self.id);
        node.write().unwrap().elements[0] = Some(element_pointer.clone());
        node.write().unwrap().keys[0] = Some(*dyn_clone::clone_box(key));

        self.key_min = Some(*dyn_clone::clone_box(key));
        self.key_max = Some(*dyn_clone::clone_box(key));
        self.element_min = Some(element_pointer.clone());
        self.element_max = Some(element_pointer.clone());
        node.write().unwrap().size = 1;

        self.elements_counter = 1;

        element_pointer
    }

    fn split_root(&mut self) -> Arc<RwLock<Node<Key, ORDER>>> {
        let new_root = Arc::new(RwLock::new(Node::new(false, None)));
        let old_root = self.root.clone();
        self.root = new_root;
        old_root.write().unwrap().parent = Some(Arc::downgrade(&self.root));
        self.root.write().unwrap().children[0] = Some(old_root);
        Node::split_child(&self.root, 0);
        self.root.clone()
    }

    fn set_extrema(&mut self, element: &Arc<RwLock<Element<Key, ORDER>>>) {
        let key_min = &self.key_min;
        let key_max = &self.key_max;

        let mut should_update_weights = false;
        {
            let key = &element.read().unwrap().key;
            if key_min.is_none() != key_max.is_none() {
                panic!("inconsistent extremas: key_min.is_none() != key_max.is_none()")
            } else if self.key_min.is_none() || self.key_max.is_none() {
                self.key_min = Some(*dyn_clone::clone_box(key));
                self.key_max = Some(*dyn_clone::clone_box(key));
                self.element_min = Some(element.clone());
                self.element_max = Some(element.clone());
                should_update_weights = true;
            } else {
                if key.partial_compare(key_min.as_ref().unwrap()) == Some(Less) {
                    self.key_min = Some(*dyn_clone::clone_box(key));
                    self.element_min = Some(element.clone());
                    should_update_weights = true;
                }
                if key.partial_compare(key_max.as_ref().unwrap()) == Some(Greater) {
                    self.key_max = Some(*dyn_clone::clone_box(key));
                    self.element_max = Some(element.clone());
                    should_update_weights = true;
                }   
            }
        }

        if should_update_weights {
            let range = self.key_min.as_ref().unwrap().distance(self.key_max.as_ref().unwrap());
            self.update_elements_weights(range as f32);
        }
    }

    fn update_elements_weights(&mut self, range: f32) {
        let mut prev_element_ptr = match &self.element_min {
            Some(e) => e.clone(),
            None => return
        };

        loop {
            let next_element_ptr;
            {
                let mut prev_element = prev_element_ptr.write().unwrap();
                let next_element_tuple = prev_element.next.clone();
                if next_element_tuple.is_none() { break }
                next_element_ptr = next_element_tuple.unwrap().0.clone().upgrade().unwrap();
                let mut next_element = next_element_ptr.write().unwrap();
                let weight = prev_element.weight(&next_element, range);
                prev_element.next.as_mut().unwrap().1 = weight;
                next_element.prev.as_mut().unwrap().1 = weight;
            }
            prev_element_ptr = next_element_ptr;
            
        }
    }

    pub fn count_elements_unique(&self) -> usize {
        let mut counter = 0usize;
        let mut element = match &self.element_min {
            Some(e) => e.clone(),
            None => return counter
        };
        loop {
            counter += 1;
            let new_element= match &element.read().unwrap().next {
                Some(e) => e.0.upgrade().unwrap().clone(),
                None => return counter
            };
            element = new_element;
        }
    }

    pub fn count_elements_agg(&self) -> usize {
        let mut counter = 0usize;
        let mut element = match &self.element_min {
            Some(e) => e.clone(),
            None => return counter
        };
        loop {
            counter += element.read().unwrap().counter;
            let new_element= match &element.read().unwrap().next {
                Some(e) => e.0.upgrade().unwrap().clone(),
                None => return counter
            };
            element = new_element;
        }
    }

    pub fn activate(
        &mut self, key: &Key, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> Result<f32> {
        let element = match self.search(key) {
            Some(e) => e,
            None => { 
                match self.data_category() {
                    DataCategory::Categorical => {
                        anyhow::bail!("activating missing categorical sensory neuron {}", key)
                    },
                    DataCategory::Numerical | DataCategory::Ordinal => {
                        if propagate_horizontal {
                            log::warn!(
                                "activating missing non-categorical sensory neuron {}, inserting",
                                key
                            );
                            self.insert(&key)
                        } else {
                            anyhow::bail!(
                                "activating missing non-categorical sensory neuron {} with {}",
                                key, 
                                "propagate_horizontal=false"
                            )
                        }
                    }
                }
            }
        };

        Ok(element.clone().write().unwrap().activate(
            signal, propagate_horizontal, propagate_vertical
        ))
    }

    pub fn deactivate(
        &mut self, key: &Key, propagate_horizontal: bool, propagate_vertical: bool
    ) -> Result<()> {
        let element = match self.search(key) {
            Some(e) => e,
            None => {
                anyhow::bail!("deactivating non-existing sensory neuron {}", key)
            }
        };

        element.write().unwrap().deactivate(propagate_horizontal, propagate_vertical);

        Ok(())
    }

    pub fn deactivate_sensor(&mut self) {
        let mut element = match &self.element_min {
            Some(e) => e.clone(),
            None => { log::warn!("no element_min in asa-graph"); return }
        };

        loop {
            element.write().unwrap().deactivate(false, false);

            let new_element= match &element.read().unwrap().next {
                Some(e) => e.0.upgrade().unwrap().clone(),
                None => break
            };
            element = new_element;
        }
    }

    
    pub fn levels(&self) 
    -> Vec<Vec<Vec<Arc<RwLock<Element<Key, ORDER>>>>>> {
        let mut ret = Vec::new();

        let mut height = 0;
        let mut node = self.root.clone();
        let mut queue: Vec<Vec<Arc<RwLock<Node<Key, ORDER>>>>> = vec![vec![]];
        queue[0].push(node.clone());

        loop {
            ret.push(vec![]);
            queue.push(vec![]);
            for i in 0..(queue[height].len()) {
                ret[height].push(vec![]);
                node = queue[height][i].clone();
                let node_lock = node.read().unwrap();
                let node_size = node_lock.size;
                for j in 0..(node_size) {
                    let element = node_lock.elements[j].as_ref().unwrap().clone();
                    ret[height][i].push(element);
                    if !node_lock.is_leaf {
                        queue[height + 1].push(node_lock.children[j].as_ref().unwrap().clone());
                    }
                }
                if !node_lock.is_leaf {
                    queue[height + 1].push(
                        node_lock.children[node_size].as_ref().unwrap().clone()
                    );
                }
            }
            if queue.last().unwrap().len() > 0 {
                height += 1;
            } else {
                break
            }
        }

        ret
    }
}

impl<'a, Key, const ORDER: usize> IntoIterator for &'a ASAGraph<Key, ORDER> 
where Key: SensorData + Sync + Send, [(); ORDER + 1]: {
    type Item = Arc<RwLock<Element<Key, ORDER>>>;
    type IntoIter = ASAGraphIntoIterator<'a, Key, ORDER>;

    fn into_iter(self) -> Self::IntoIter {
        ASAGraphIntoIterator {
            graph: self,
            index: match self.element_min.clone() {
                Some(element_min) => Some(element_min.clone()),
                None => None
            },
        }
    }
}

pub struct ASAGraphIntoIterator<'a, Key, const ORDER: usize = 25>
where Key: SensorData + Sync + Send, [(); ORDER + 1]: {
    graph: &'a ASAGraph<Key, ORDER>,
    index: Option<Arc<RwLock<Element<Key, ORDER>>>>
}

impl<'a, Key, const ORDER: usize> Iterator for ASAGraphIntoIterator<'a, Key, ORDER> 
where Key: SensorData + Sync + Send, [(); ORDER + 1]: {
    type Item = Arc<RwLock<Element<Key, ORDER>>>;
    fn next(&mut self) -> Option<Arc<RwLock<Element<Key, ORDER>>>> {
        let next_option;
        let result = match self.index.clone() {
            Some(element) => {
                next_option = match &element.read().unwrap().next {
                    Some(next_tuple) => match next_tuple.0.upgrade() {
                        Some(next) => Some(next),
                        None => None
                    },
                    None => None
                };

                Some(element)
            },
            None => return None
        };

        self.index = next_option;

        result
    }
}

impl<Key, const ORDER: usize> Display for ASAGraph<Key, ORDER> 
where Key: SensorData + Sync + Send, [(); ORDER + 1]: {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut height = 0;
        let mut node = self.root.clone();
        let mut queue: Vec<Vec<Arc<RwLock<Node<Key, ORDER>>>>> = vec![vec![]];
        queue[0].push(node.clone());

        loop {
            queue.push(vec![]);
            for i in 0..(queue[height].len()) {
                node = queue[height][i].clone();
                let node_size = node.read().unwrap().size;
                write!(f, "||")?;
                for j in 0..(node_size) {
                    let element = node.read().unwrap().elements[j].as_ref().unwrap().clone();
                    write!(f, "{}:{}|", &element.read().unwrap().key, element.read().unwrap().counter)?;
                    if !node.read().unwrap().is_leaf {
                        queue[height + 1].push(node.read().unwrap().children[j].as_ref().unwrap().clone());
                    }
                }
                if !node.read().unwrap().is_leaf {
                    queue[height + 1].push(node.read().unwrap().children[node_size].as_ref().unwrap().clone());
                }
                write!(f, "| ")?;
            }
            if queue.last().unwrap().len() > 0 {
                height += 1;
                writeln!(f, "")?;
            } else {
                writeln!(f, "")?;
                return Ok(())
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use rand::Rng;
    use std::{ time::Instant };

    use super::ASAGraph;

    #[test]
    fn create_empty_graph() {
        ASAGraph::<i32, 3>::new(1);
    }

    #[test]
    fn create_1000_elements_graph() {
        let mut rng = rand::thread_rng();

        let start = Instant::now();

        let graph = ASAGraph::<i32, 3>::new_arc(1);

        let n = 1000;
        for _ in (0..n).rev() {
            let random_number: i32 = rng.gen_range(0..10000);
            graph.write().unwrap().insert(&random_number);
        }

        let duration = start.elapsed();

        println!("Time elapsed for ASAGraph insertion of {} elements is is: {:?}", n, duration);
    }

    #[test]
    fn print_graph() {
        let mut rng = rand::thread_rng();

        let mut graph = ASAGraph::<i32, 5>::new(1);

        for _ in 0..50 {
            let number: i32 = rng.gen_range(1..=20);
            graph.insert(&number);
        }

        graph.print_graph();
    }

    #[test]
    fn insert_3_degree() {
        let mut graph = ASAGraph::<i32, 3>::new(1);

        for i in 1..=250 {
            graph.insert(&i);
        }

        for i in (150..=500).rev() {
            graph.insert(&i);
        }

        assert_eq!(graph.count_elements_unique(), 500);
        assert_eq!(graph.count_elements_agg(), 601);

        let root_first_key = graph.root.read().unwrap().elements[0].as_ref().unwrap().read().unwrap().key;
        assert_eq!(root_first_key, 128);
        assert_eq!(graph.key_min.unwrap(), 1);
        assert_eq!(graph.element_min.as_ref().unwrap().read().unwrap().key, 1);
        assert_eq!(graph.key_max.unwrap(), 500);
        assert_eq!(graph.element_max.as_ref().unwrap().read().unwrap().key, 500);

        graph.print_graph();
    }

    #[test]
    fn insert_25_degree() {
        let mut graph = ASAGraph::<i32, 25>::new(1);

        for i in 1..=250 {
            graph.insert(&i);
        }

        for i in (150..=500).rev() {
            graph.insert(&i);
        }

        assert_eq!(graph.count_elements_unique(), 500);
        assert_eq!(graph.count_elements_agg(), 601);

        let root_first_key = graph.root.read().unwrap().elements[0].as_ref().unwrap().read().unwrap().key;
        assert_eq!(root_first_key, 169);
        assert_eq!(graph.key_min.unwrap(), 1);
        assert_eq!(graph.element_min.as_ref().unwrap().read().unwrap().key, 1);
        assert_eq!(graph.key_max.unwrap(), 500);
        assert_eq!(graph.element_max.as_ref().unwrap().read().unwrap().key, 500);

        graph.print_graph();
    }

    #[test]
    fn search() {
        let mut graph = ASAGraph::<i32, 3>::new(1);

        let n = 100;
        for i in 0..n {
            graph.insert(&i);
        }

        for i in 0..n {
            let result = graph.search(&i);
            assert!(result.is_some());
            assert_eq!(result.unwrap().read().unwrap().key, i);
        }
        
        assert!(graph.search(&101).is_none());
        assert!(graph.search(&-1).is_none());
    }

    #[test]
    fn test_connections() {
        let mut graph = ASAGraph::<i32, 3>::new(1);
    
        let n = 50;
        for i in 1..=n {
            graph.insert(&i);
        }

        let mut prev_element;
        let mut current_element = graph.element_min.as_ref().unwrap().clone();
        for i in 1..=n {
            assert_eq!(current_element.read().unwrap().key, i);
            {
                let prev = &current_element.read().unwrap().prev;
                let next = &current_element.read().unwrap().next;
                if i == 1 { 
                    assert!(prev.is_none());
                    assert_eq!(next.as_ref().unwrap().0.upgrade().unwrap().read().unwrap().key, 2);
                } else if i == n {
                    assert_eq!(prev.as_ref().unwrap().0.upgrade().unwrap().read().unwrap().key, n - 1);
                    assert!(next.is_none());
                    break
                } else {
                    assert_eq!(prev.as_ref().unwrap().0.upgrade().unwrap().read().unwrap().key, i - 1);
                    assert_eq!(next.as_ref().unwrap().0.upgrade().unwrap().read().unwrap().key, i + 1);
                }
            }
            prev_element = current_element.clone();
            current_element = prev_element.read().unwrap().next.as_ref().unwrap().0.upgrade().unwrap().clone();
        }
    }

    #[test]
    fn test_connections_rev() {
        let mut graph = ASAGraph::<i32, 3>::new(1);
    
        let n = 50;
        for i in (1..=n).rev() {
            graph.insert(&i);
        }

        let mut prev_element;
        let mut current_element = graph.element_min.as_ref().unwrap().clone();
        for i in 1..=n {
            assert_eq!(current_element.read().unwrap().key, i);
            {
                let prev = &current_element.read().unwrap().prev;
                let next = &current_element.read().unwrap().next;
                if i == 1 { 
                    assert!(prev.is_none());
                    assert_eq!(next.as_ref().unwrap().0.upgrade().unwrap().read().unwrap().key, 2);
                } else if i == n {
                    assert_eq!(prev.as_ref().unwrap().0.upgrade().unwrap().read().unwrap().key, n - 1);
                    assert!(next.is_none());
                    break
                } else {
                    assert_eq!(prev.as_ref().unwrap().0.upgrade().unwrap().read().unwrap().key, i - 1);
                    assert_eq!(next.as_ref().unwrap().0.upgrade().unwrap().read().unwrap().key, i + 1);
                }
            }
            prev_element = current_element.clone();
            current_element = prev_element.read().unwrap().next.as_ref().unwrap().0.upgrade().unwrap().clone();
        }
    }

    #[test]
    fn iterator_test() {
        let mut graph = ASAGraph::<i32, 3>::new(1);
        let n = 50;
        for i in (0..=n).rev() { graph.insert(&i); }
        for (i, element) in graph.into_iter().enumerate() {
            assert_eq!(element.read().unwrap().key, i as i32);
        }
        assert_eq!(graph.key_min.unwrap(), 0i32);
    }

    #[test]
    fn new_from_vec() {
        let vec = vec!["kot".to_string(), "pies".to_string()];
        let graph = ASAGraph::<_, 25>::new_rc_from_vec(1, &vec[..]);
        assert!(graph.read().unwrap().search(&"kot".to_string()).is_some());
        let graph = ASAGraph::<_, 3>::new_from_vec(1, &vec[..]);
        assert!(graph.search(&"pies".to_string()).is_some());
    }

    #[test]
    fn ids() {
        let mut graph = ASAGraph::<i32, 3>::new(1);
        for i in 1..=25 { graph.insert(&i); }
        for i in 1..=25 { assert_eq!(i as u32, graph.search(&i).unwrap().read().unwrap().id) }
    }

    #[test]
    fn display_graph() {
        let mut graph = ASAGraph::<i32, 3>::new(1);
        for i in 1..=25 { graph.insert(&i); }

        println!("{graph}");
    }
}