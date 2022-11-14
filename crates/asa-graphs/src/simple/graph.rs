use std::{
    fmt::Display,
    rc::Rc,
    cell::RefCell
};

use witchnet_common::distances::Distance;

use super::{
    element::Element,
    node::Node
};

#[derive(Clone, Debug)]
pub struct ASAGraph<Key, const ORDER: usize = 25>
where Key: Clone + Display + PartialOrd + PartialEq + Distance, [(); ORDER + 1]: {
    pub name: String,
    pub root: Rc<RefCell<Node<Key, ORDER>>>,
    pub(crate) element_min: Option<Rc<RefCell<Element<Key, ORDER>>>>,
    pub(crate) element_max: Option<Rc<RefCell<Element<Key, ORDER>>>>,
    pub key_min: Option<Key>,
    pub key_max: Option<Key>
}

impl<Key, const ORDER: usize> ASAGraph<Key, ORDER> 
where Key: Clone + Display + PartialOrd + PartialEq + Distance, [(); ORDER + 1]: {
    pub fn new(name: &str) -> ASAGraph<Key, ORDER> {
        if ORDER < 3 {
            panic!("Graph order must be >= 3");
        }
        ASAGraph {
            name: name.to_string(),
            root: Rc::new(RefCell::new(Node::<Key, ORDER>::new(true, None))),
            element_min: None,
            element_max: None,
            key_min: None,
            key_max: None
        }
    }

    pub fn search(&self, key: &Key) -> Option<Rc<RefCell<Element<Key, ORDER>>>> {
        let node = &self.root;
        
        let (key_min, key_max) = self.extreme_keys()?;

        if key.distance(key_max) > key.distance(key_min) {
            return Self::search_left(key, &*node.borrow())
        } else {
            return Self::search_right(key, &*node.borrow())
        }
    }

    fn search_left<'a, 'b>(
        key: &'a Key, mut node: &'b Node<Key, ORDER>
    ) -> Option<Rc<RefCell<Element<Key, ORDER>>>> {
        loop {
            let mut index = 0;
            {
                let mut current_key = node.keys[index].as_ref().unwrap();
                
                while index < node.size && key > current_key {
                    index += 1;
                    if index < node.size {
                        current_key = node.keys[index].as_ref().unwrap();
                    }
                }

                if index < node.size && key == current_key {
                    let element = node.elements[index].as_ref().unwrap().clone();
                    return Some(element)
                } else if node.is_leaf {
                    return None
                }
            }
                
            let node_ptr = node.children[index].as_ref().unwrap();
            unsafe { node = node_ptr.try_borrow_unguarded().unwrap() };
        }
    }

    fn search_right<'a, 'b>(
        key: &'a Key, mut node: &'b Node<Key, ORDER>
    ) -> Option<Rc<RefCell<Element<Key, ORDER>>>> {
        loop {
            let mut index = node.size - 1;
            {
                let mut current_key = node.keys[index].as_ref().unwrap();
                
                while index > 0 && key < current_key {
                    index -= 1;
                    current_key = node.keys[index].as_ref().unwrap();
                }

                if key == current_key {
                    let element = node.elements[index].as_ref().unwrap().clone();
                    return Some(element)
                } else if node.is_leaf {
                    return None
                } else if key > current_key {
                    index += 1;
                }
            }
            let node_ptr = node.children[index].as_ref().unwrap();
            unsafe { node = node_ptr.try_borrow_unguarded().unwrap() };
        }
    }

    pub fn insert(&mut self, key: &Key) -> Rc<RefCell<Element<Key, ORDER>>> {
        let mut node = self.root.clone();

        if node.borrow().size == 0 { return self.insert_first_element(&node, key) }

        if node.borrow().size == Node::<Key, ORDER>::MAX_KEYS { node = self.split_root(); }

        let (key_min, key_max) = self.extreme_keys().unwrap_or_else(|| {
            panic!("element_min / element_min must not be nullptr")
        });

        loop {
            let node_insert_result = if key.distance(key_max) > key.distance(key_min) {
                node.borrow().insert_existing_key(key, true)
            } else {
                node.borrow().insert_existing_key(key, false)
            };
            if let Some(el) = node_insert_result.0 { return el }
            let mut index = node_insert_result.1;
    
            if node.borrow().is_leaf {
                let element = Node::insert_key_leaf(&node, key);
                self.set_extrema(&element);
                return element
            } else {
                let child_size = node.borrow().children[index].as_ref().unwrap().borrow().size;
                if child_size == Node::<Key, ORDER>::MAX_KEYS {
                    Node::split_child(&node, index);
                    if key > &node.borrow().elements[index].as_ref().unwrap().borrow().key {
                        index += 1 
                    } else if key == node.borrow().keys[index].as_ref().unwrap() {
                        return node.borrow().elements[index].as_ref().unwrap().clone()
                    }
                }
                let new_node = node.borrow().children[index].as_ref().unwrap().clone();
                node = new_node.clone();
            }
        }
    }

    pub fn print_graph(&self) {
        let mut height = 0;
        let mut node = self.root.clone();
        let mut queue: Vec<Vec<Rc<RefCell<Node<Key, ORDER>>>>> = vec![vec![]];
        queue[0].push(node.clone());

        loop {
            queue.push(vec![]);
            for i in 0..(queue[height].len()) {
                node = queue[height][i].clone();
                let node_size = node.borrow().size;
                print!("||");
                for j in 0..(node_size) {
                    let element = node.borrow().elements[j].as_ref().unwrap().clone();
                    print!("{}:{}|", &element.borrow().key, element.borrow().counter);
                    if !node.borrow().is_leaf {
                        queue[height + 1].push(node.borrow().children[j].as_ref().unwrap().clone());
                    }
                }
                if !node.borrow().is_leaf {
                    queue[height + 1].push(node.borrow().children[node_size].as_ref().unwrap().clone());
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
        &mut self, node: &Rc<RefCell<Node<Key, ORDER>>>,  key: &Key
    ) -> Rc<RefCell<Element<Key, ORDER>>> {
        let element_pointer = Rc::new(RefCell::new(Element::<Key, ORDER>::new(key)));
        node.borrow_mut().elements[0] = Some(element_pointer.clone());
        node.borrow_mut().keys[0] = Some(key.clone());

        self.key_min = Some(key.clone());
        self.key_max = Some(key.clone());
        self.element_min = Some(element_pointer.clone());
        self.element_max = Some(element_pointer.clone());
        node.borrow_mut().size = 1;

        element_pointer
    }

    fn split_root(&mut self) -> Rc<RefCell<Node<Key, ORDER>>> {
        let new_root = Rc::new(RefCell::new(Node::new(false, None)));
        let old_root = self.root.clone();
        self.root = new_root;
        old_root.borrow_mut().parent = Some(Rc::downgrade(&self.root));
        self.root.borrow_mut().children[0] = Some(old_root);
        Node::split_child(&self.root, 0);
        self.root.clone()
    }

    fn set_extrema(&mut self, element: &Rc<RefCell<Element<Key, ORDER>>>) {
        let key = &element.borrow().key;
        let key_min = &self.key_min;
        let key_max = &self.key_max;
        if key_min.is_none() != key_max.is_none() {
            panic!("inconsistent extremas: key_min.is_none() != key_max.is_none()")
        } else if self.key_min.is_none() || self.key_max.is_none() {
            self.key_min = Some(key.clone());
            self.key_max = Some(key.clone());
            self.element_min = Some(element.clone());
            self.element_max = Some(element.clone());
        } else {
            if key < key_min.as_ref().unwrap() {
                self.key_min = Some(key.clone());
                self.element_min = Some(element.clone());
            }
            if key > key_max.as_ref().unwrap() {
                self.key_max = Some(key.clone());
                self.element_max = Some(element.clone());
            }   
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
            let new_element= match &element.borrow().next {
                Some(e) => e.upgrade().unwrap().clone(),
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
            counter += element.borrow().counter;
            let new_element= match &element.borrow().next {
                Some(e) => e.upgrade().unwrap().clone(),
                None => return counter
            };
            element = new_element;
        }
    }

    pub fn levels(&self) -> Vec<Vec<Vec<Rc<RefCell<Element<Key, ORDER>>>>>> {
        let mut ret = Vec::new();

        let mut height = 0;
        let mut node = self.root.clone();
        let mut queue: Vec<Vec<Rc<RefCell<Node<Key, ORDER>>>>> = vec![vec![]];
        queue[0].push(node.clone());

        loop {
            ret.push(vec![]);
            queue.push(vec![]);
            for i in 0..(queue[height].len()) {
                ret[height].push(vec![]);
                node = queue[height][i].clone();
                let node_lock = node.borrow();
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

#[cfg(test)]
pub mod tests {
    use rand::Rng;
    use std::time::Instant;
    
    use super::ASAGraph;

    #[test]
    fn create_empty_graph() {
        ASAGraph::<i32, 3>::new("test");
    }

    #[test]
    fn create_100_elements_graph() {
        let mut rng = rand::thread_rng();

        let start = Instant::now();

        let mut graph = Box::new(ASAGraph::<i32, 3>::new("test"));

        let n = 1_000;
        for _ in 0..n {
            let number: i32 = rng.gen();
            graph.insert(&number);
        }

        let duration = start.elapsed();

        println!("Time elapsed for ASAGraph insertion of {} elements is is: {:?}", n, duration);
    }

    #[test]
    fn print_graph() {
        let mut rng = rand::thread_rng();

        let mut graph = ASAGraph::<i32, 5>::new("test");

        for _ in 0..50 {
            let number: i32 = rng.gen_range(1..=20);
            graph.insert(&number);
        }

        graph.print_graph();
    }

    #[test]
    fn insert_3_degree() {
        let mut graph = ASAGraph::<i32, 3>::new("test");

        for i in 1..=250 {
            graph.insert(&i);
        }

        for i in (150..=500).rev() {
            graph.insert(&i);
        }

        assert_eq!(graph.count_elements_unique(), 500);
        assert_eq!(graph.count_elements_agg(), 601);

        let root_first_key = graph.root.borrow().elements[0].as_ref().unwrap().borrow().key;
        assert_eq!(root_first_key, 128);
        assert_eq!(graph.key_min.unwrap(), 1);
        assert_eq!(graph.element_min.as_ref().unwrap().borrow().key, 1);
        assert_eq!(graph.key_max.unwrap(), 500);
        assert_eq!(graph.element_max.as_ref().unwrap().borrow().key, 500);

        graph.print_graph();
    }

    #[test]
    fn insert_25_degree() {
        let mut graph = ASAGraph::<i32, 25>::new("test");

        for i in 1..=250 {
            graph.insert(&i);
        }

        for i in (150..=500).rev() {
            graph.insert(&i);
        }

        assert_eq!(graph.count_elements_unique(), 500);
        assert_eq!(graph.count_elements_agg(), 601);

        let root_first_key = graph.root.borrow().elements[0].as_ref().unwrap().borrow().key;
        assert_eq!(root_first_key, 169);
        assert_eq!(graph.key_min.unwrap(), 1);
        assert_eq!(graph.element_min.as_ref().unwrap().borrow().key, 1);
        assert_eq!(graph.key_max.unwrap(), 500);
        assert_eq!(graph.element_max.as_ref().unwrap().borrow().key, 500);

        graph.print_graph();
    }

    #[test]
    fn search() {
        let mut graph = ASAGraph::<i32, 3>::new("test");

        let n = 100;
        for i in 0..n {
            graph.insert(&i);
        }

        for i in 0..n {
            let result = graph.search(&i);
            assert!(result.is_some());
            assert_eq!(result.unwrap().borrow().key, i);
        }
        
        assert!(graph.search(&101).is_none());
        assert!(graph.search(&-1).is_none());
    }

    #[test]
    fn test_connections() {
        let mut graph = ASAGraph::<i32, 3>::new("test");
    
        let n = 50;
        for i in 1..=n {
            graph.insert(&i);
        }

        let mut prev_element;
        let mut current_element = graph.element_min.as_ref().unwrap().clone();
        for i in 1..=n {
            assert_eq!(current_element.borrow().key, i);
            {
                let prev = &current_element.borrow().prev;
                let next = &current_element.borrow().next;
                if i == 1 { 
                    assert!(prev.is_none());
                    assert_eq!(next.as_ref().unwrap().upgrade().unwrap().borrow().key, 2);
                } else if i == n {
                    assert_eq!(prev.as_ref().unwrap().upgrade().unwrap().borrow().key, n - 1);
                    assert!(next.is_none());
                    break
                } else {
                    assert_eq!(prev.as_ref().unwrap().upgrade().unwrap().borrow().key, i - 1);
                    assert_eq!(next.as_ref().unwrap().upgrade().unwrap().borrow().key, i + 1);
                }
            }
            prev_element = current_element.clone();
            current_element = prev_element.borrow().next.as_ref().unwrap().upgrade().unwrap().clone();
        }
    }

    #[test]
    fn test_connections_rev() {
        let mut graph = ASAGraph::<i32, 3>::new("test");
    
        let n = 50;
        for i in (1..=n).rev() {
            graph.insert(&i);
        }

        let mut prev_element;
        let mut current_element = graph.element_min.as_ref().unwrap().clone();
        for i in 1..=n {
            assert_eq!(current_element.borrow().key, i);
            {
                let prev = &current_element.borrow().prev;
                let next = &current_element.borrow().next;
                if i == 1 { 
                    assert!(prev.is_none());
                    assert_eq!(next.as_ref().unwrap().upgrade().unwrap().borrow().key, 2);
                } else if i == n {
                    assert_eq!(prev.as_ref().unwrap().upgrade().unwrap().borrow().key, n - 1);
                    assert!(next.is_none());
                    break
                } else {
                    assert_eq!(prev.as_ref().unwrap().upgrade().unwrap().borrow().key, i - 1);
                    assert_eq!(next.as_ref().unwrap().upgrade().unwrap().borrow().key, i + 1);
                }
            }
            prev_element = current_element.clone();
            current_element = prev_element.borrow().next.as_ref().unwrap().upgrade().unwrap().clone();
        }
    }
}