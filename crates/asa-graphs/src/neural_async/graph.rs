use std::{
    sync::{ Arc, RwLock },
    cmp::Ordering::*,
    marker::PhantomData,
    fmt::{ Display, Formatter, Result as FmtResult },
    io::{ self, Write }
};

use anyhow::Result;

use witchnet_common::{ 
    sensor::SensorData,
    neuron::NeuronAsync,
    data::{ DataCategory, DataType, DataDeductor, DataTypeValue }, connection::collective::defining::{ConstantOneWeightAsync, DefiningWeightingStrategyAsync},
};

use super::{
    element::Element,
    node::Node
};

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
    
    pub fn new_from_vec_custom(
        id: u32,
        data: &[Key],
        weighting_strategy: Arc<dyn DefiningWeightingStrategyAsync>,
        interelement_activation_threshold: f32,
        interelement_activation_exponent: i32
    ) -> Self {
        let mut graph = Self::new(id);
        for point in data { graph.insert_custom(
            point, 
            weighting_strategy.clone(),
            interelement_activation_threshold,
            interelement_activation_exponent
        ); }
        graph
    }

    pub fn new_rc_from_vec(id: u32, data: &[Key]) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new_from_vec(id, data)))
    }
    
    pub fn new_rc_from_vec_custom(
        id: u32, 
        data: &[Key], 
        weighting_strategy: Arc<dyn DefiningWeightingStrategyAsync>,
        interelement_activation_threshold: f32,
        interelement_activation_exponent: i32
    ) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new_from_vec_custom(
            id, 
            data, 
            weighting_strategy,
            interelement_activation_threshold,
            interelement_activation_exponent
        )))
    }

    pub fn new_box_from_vec(id: u32, data: &[Key]) -> Box<Self> {
        Box::new(Self::new_from_vec(id, data))
    }
    
    pub fn new_box_from_vec_custom(
        id: u32, 
        data: &[Key], 
        weighting_strategy: Arc<dyn DefiningWeightingStrategyAsync>,
        interelement_activation_threshold: f32,
        interelement_activation_exponent: i32
    ) -> Box<Self> {
        Box::new(Self::new_from_vec_custom(
            id, 
            data, 
            weighting_strategy,
            interelement_activation_threshold,
            interelement_activation_exponent
        ))
    }
    
    pub fn id(&self) -> u32 { self.id }

    pub fn data_type(&self) -> DataType { self.data_type.data_type() }

    pub fn data_category(&self) -> DataCategory { self.data_type.data_category() }

    pub fn fuzzy_search(
        &mut self, item: &Key, threshold: f32, perserve_inserted_neuron: bool
    ) -> Option<(Arc<RwLock<Element<Key, ORDER>>>, f32)> {
        if let Some(element) = self.search(item) {
            return Some((element, 1.0))
        } else {
            if self.data_category().is_categorical() { return None }

            let element_ptr = self.insert(item);
            let mut element = element_ptr.write().unwrap();
            element.decrement_counter();

            if let Some(next) = &element.next {
                let next_element = next.0.upgrade().unwrap();
                let next_weight = next.1;
                if let Some(prev) = &element.prev {
                    let prev_element = prev.0.upgrade().unwrap();
                    let prev_weight = prev.1;
                    if next_weight > prev_weight {
                        if next_weight >= threshold { 
                            Some((next_element, next_weight)) 
                        } else {
                            if perserve_inserted_neuron { 
                                Some((element_ptr.clone(), 1.0)) 
                            } else { None } 
                        }
                    } else {
                        if prev_weight >= threshold { 
                            Some((prev_element, prev_weight))
                        } else { 
                            if perserve_inserted_neuron { 
                                Some((element_ptr.clone(), 1.0)) 
                            } else { None } 
                        }
                    }
                } else {
                    if next_weight >= threshold { 
                        Some((next_element, next_weight))
                    } else {
                        if perserve_inserted_neuron { 
                            Some((element_ptr.clone(), 1.0)) 
                        } else { None } 
                    }
                }
            } else {
                if let Some(prev) = &element.prev {
                    let prev_element = prev.0.upgrade().unwrap();
                    let prev_weight = prev.1;
                    if prev_weight >= threshold { 
                        Some((prev_element, prev_weight))
                    } else {
                        if perserve_inserted_neuron { 
                            Some((element_ptr.clone(), 1.0)) 
                        } else { None } 
                    }
                } else {
                    if perserve_inserted_neuron { 
                        Some((element_ptr.clone(), 1.0)) 
                    } else { None } 
                }
            }
        }
    }

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
        self.insert_custom(key, Arc::new(ConstantOneWeightAsync), 0.00001, 1)
    }

    pub fn insert_custom(
        &mut self, 
        key: &Key, 
        weighting_strategy: Arc<dyn DefiningWeightingStrategyAsync>,
        interelement_activation_threshold: f32,
        interelement_activation_exponent: i32
    ) -> Arc<RwLock<Element<Key, ORDER>>> {
        let mut node = self.root.clone();

        if node.read().unwrap().size == 0 { return self.insert_first_element(
            &node, 
            key, 
            weighting_strategy, 
            interelement_activation_threshold,
            interelement_activation_exponent
        ) }

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
                let element = Node::insert_key_leaf(
                    &node,
                    key,
                    element_id,
                    self.id,
                    self.range(),
                    weighting_strategy, 
                    interelement_activation_threshold,
                    interelement_activation_exponent
                );
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

    pub fn remove(&mut self, key: &Key) -> bool {
        let (key_min, key_max) = match self.extreme_keys(){
            Some((key_min, key_max)) => (
                *dyn_clone::clone_box(key_min), 
                *dyn_clone::clone_box(key_max)
            ),
            None => return false
        };
        
        let result = self.remove_without_weights(key);

        if key.equals(&key_min) || key.equals(&key_max) {
            // self.update_elements_weights(key_min.distance(&key_max) as f32);
        }

        result
    }

    pub fn remove_without_weights(&mut self, key: &Key) -> bool {
        // 1. Use the search operation to find an element containing the key intended for removal.
        let mut current_node = Some(self.root.clone());
        if current_node.as_ref().unwrap().read().unwrap().size == 0 { return false }

        let mut parent: Option<Arc<RwLock<Node<Key, ORDER>>>> = None;
        let mut parent_of_parent: Option<Arc<RwLock<Node<Key, ORDER>>>> = None;
        let mut parent_index: Option<usize> = None;
        let mut child_index: Option<usize> = None;

        while let Some(node) = &current_node.clone() {
            let mut index = 0;
            {
                let node = node.read().unwrap();
                while index < node.size && key.partial_compare(
                    node.keys[index].as_ref().unwrap()
                ) == Some(Greater) { index += 1; }
            }

            if index < node.read().unwrap().size && key.equals(node.read().unwrap().keys[index].as_ref().unwrap()) {
                let element = node.read().unwrap().elements[index].as_ref().unwrap().clone();
                if element.read().unwrap().counter == 1 {
                    // 3. I f the element storing the removed key is a leaf
                    if node.read().unwrap().is_leaf {
                        // 3. remove the element storing this key from this leaf, switch pointers
                        //    from its predecessor and successor to point themselves 
                        //    as direct neighbors.
                        node.write().unwrap().remove_element(index, self.range());
                         // 3. Next, if this leaf is not empty, finish the remove operation, 
                         //    else go to step 6 to fill or remove this empty leaf
                        if node.read().unwrap().size > 0 {
                            return true
                        } else {
                            // 6. Empty leaf
                            if !Arc::ptr_eq(&node, &self.root) {
                                self.empty_node(
                                    parent.unwrap().clone(),
                                    child_index.unwrap(),
                                    parent_of_parent,
                                    parent_index
                                );
                            }
                            return true
                        }
                    } else {
                        // 4. Else the element storing the removed key is a non leaf node that 
                        //    must be replaced by the previous or next element stored in leaves
                        let mut left_leaf = node.read().unwrap().children[index].clone();
                        let mut right_leaf = node.read().unwrap().children[index + 1].clone();
                        let mut left_leaf_parent = Some(node.clone());
                        let mut right_leaf_parent = Some(node.clone());
                        let mut left_leaf_parent_of_parent = parent.clone();
                        let mut right_leaf_parent_of_parent = parent.clone();
                        let mut left_parent_size = node.read().unwrap().size;
                        let mut right_parent_size = node.read().unwrap().size;

                        while !left_leaf.as_ref().unwrap().read().unwrap().is_leaf {
                            left_parent_size = left_leaf.as_ref().unwrap().read().unwrap().size;
                            right_parent_size = right_leaf.as_ref().unwrap().read().unwrap().size;
                            left_leaf_parent_of_parent = left_leaf_parent.take();
                            right_leaf_parent_of_parent = right_leaf_parent.take();
                            left_leaf_parent = left_leaf.clone();
                            right_leaf_parent = right_leaf.clone();
                            left_leaf = {
                                let left_leaf_deref = left_leaf.as_ref().unwrap().read().unwrap();
                                left_leaf_deref.children[left_leaf_deref.size].clone()
                            };
                            right_leaf = {
                                let right_leaf_deref = right_leaf.as_ref().unwrap().read().unwrap();
                                right_leaf_deref.children[0].clone()
                            };
                        }
                        // 4. If one of these leaves contains more than one element, 
                        //    replace the removed element in the non leaf node by this
                        //    connected neighbor element from the leaf containing 
                        //    more than one element, and finish the remove operation
                        let left_leaf_size = left_leaf.as_ref().unwrap().read().unwrap().size;
                        let right_leaf_size = right_leaf.as_ref().unwrap().read().unwrap().size;
                        if left_leaf_size >= 2 {
                            let mut node_deref = node.write().unwrap();
                            let mut left_leaf = left_leaf.as_mut().unwrap().write().unwrap();
                            node_deref.remove_element_without_shift(index, self.range());
                            node_deref.keys[index] = left_leaf.keys[left_leaf_size - 1].take();
                            node_deref.elements[index] = left_leaf.elements[left_leaf_size - 1].take();
                            left_leaf.size -= 1;
                            return true
                        } else if right_leaf_size >= 2 {
                            let mut node_deref = node.write().unwrap();
                            let mut right_leaf = right_leaf.as_mut().unwrap().write().unwrap();
                            node_deref.remove_element_without_shift(index, self.range());
                            node_deref.keys[index] = right_leaf.keys[0].take();
                            node_deref.elements[index] = right_leaf.elements[0].take();
                            right_leaf.remove_element_soft(0);
                            return true
                        } else {
                            // 5. Therefore the previous and next elements are stored in 
                            //    leaves that contain only single elements
                            //    choose the element of the leaf which parent contains 
                            //    more elements to simplify the next rebalancing operation.
                            if left_parent_size >= right_parent_size {
                                {
                                    let mut left_leaf = left_leaf.as_mut().unwrap().write().unwrap();
                                    let mut node_deref = node.write().unwrap();
                                    node_deref.elements[index].as_mut().unwrap().write().unwrap()
                                        .remove_connections(self.range());
                                    node_deref.keys[index] = left_leaf.keys[0].take();
                                    node_deref.elements[index] = left_leaf.elements[0].take();
                                    left_leaf.size -= 1;
                                }
                                // 6. Empty leaf
                                let left_leaf_parent_ptr = left_leaf_parent.as_ref().unwrap();
                                let leaf_index = if Arc::ptr_eq(left_leaf_parent_ptr, node) { 
                                    index 
                                } else {
                                    match left_leaf_parent_ptr.read() {
                                        Ok(b) => b.size,
                                        Err(_) => node.read().unwrap().size,
                                    }
                                };
                                let leaf_parent_index = if left_leaf_parent_of_parent.is_some() &&
                                    Arc::ptr_eq(
                                        left_leaf_parent_of_parent.as_ref().unwrap(), &node
                                    ) { Some(index) } else {
                                    if left_leaf_parent_of_parent.is_some() && 
                                        parent.is_some() && Arc::ptr_eq(
                                            left_leaf_parent_of_parent.as_ref().unwrap(), 
                                            &parent.as_ref().unwrap()
                                    ) { child_index } else {
                                        if left_leaf_parent_of_parent.is_some() {
                                            Some(
                                                left_leaf_parent_of_parent.as_ref()
                                                    .unwrap().read().unwrap().size
                                            )
                                        } else { None }
                                    }
                                };
                                self.empty_node(
                                    left_leaf_parent.unwrap().clone(),
                                    leaf_index,
                                    left_leaf_parent_of_parent,
                                    leaf_parent_index
                                );
                                return true
                            } else {
                                {
                                    let mut right_leaf = right_leaf.as_mut().unwrap().write().unwrap();
                                    let mut node_deref = node.write().unwrap();
                                    node_deref.elements[index].as_mut().unwrap().write().unwrap()
                                        .remove_connections(self.range());
                                    node_deref.keys[index] = right_leaf.keys[0].take();
                                    node_deref.elements[index] = right_leaf.elements[0].take();
                                    right_leaf.size -= 1;
                                }
                                // 6. Empty leaf
                                let leaf_index = if Arc::ptr_eq(
                                    left_leaf_parent.as_ref().unwrap(), &node
                                ) { index + 1 } else { 0 };
                                let leaf_parent_index = if left_leaf_parent_of_parent.is_some() &&
                                    Arc::ptr_eq(
                                        left_leaf_parent_of_parent.as_ref().unwrap(), &node
                                ) { Some(index + 1) } else {
                                    if left_leaf_parent_of_parent.is_some() && 
                                        parent.is_some() && Arc::ptr_eq(
                                            left_leaf_parent_of_parent.as_ref().unwrap(), 
                                            &parent.as_ref().unwrap()
                                    ) { child_index } else { 
                                        if left_leaf_parent_of_parent.is_some() { Some(0) } else { None }
                                    }
                                };
                                self.empty_node(
                                    right_leaf_parent.unwrap().clone(), 
                                    leaf_index, 
                                    right_leaf_parent_of_parent, 
                                    leaf_parent_index
                                );
                                return true
                            }
                        }
                    }
                } else {
                    // 2. I f the counter of the element storing the removed key 
                    //    is greater than one, decrement this counter ,
                    //    remove the link to the b oun d object, and finish the remove operation.
                    element.write().unwrap().counter -= 1;
                    return true
                }
            } else if node.read().unwrap().is_leaf {
                //  1. If this key is not found, finish this operation with no effect
                return false
            } else {
                parent_of_parent = parent;
                parent = Some(node.clone());
                current_node = node.read().unwrap().children[index].clone();
                parent_index = child_index;
                child_index = Some(index);
            }
        }

        false
    }

    fn empty_node(
        &mut self, 
        node: Arc<RwLock<Node<Key, ORDER>>>,
        index: usize,
        parent: Option<Arc<RwLock<Node<Key, ORDER>>>>,
        parent_index: Option<usize>
    ) {
        // 6. If one of the nearest siblings of the empty leaf contains 
        //    more than one element, replace its ancestor element from
        //    the empty leaf side by the closest element from this sibling
        //    and shift this ancestor element to the empty node
        //    Next, finish the remove operation
        let empty_node = node.read().unwrap().children[index].as_ref().unwrap().clone();
        let sibling_node = node.read().unwrap().children[1].clone();
        let second_sibling_node = if index == 0 {
            None
        } else { node.read().unwrap().children[index - 1].clone() };
        
        let mut node_deref = node.write().unwrap();
        let mut empty_node_deref = empty_node.write().unwrap();
        let node_size = node_deref.size;
        
        if index == 0 && sibling_node.as_ref().unwrap().read().unwrap().size > 1 {
            let mut sibling_node_deref = sibling_node.as_ref().unwrap().write().unwrap();
            
            empty_node_deref.keys[0] = node_deref.keys[0].take();
            empty_node_deref.elements[0] = node_deref.elements[0].take();
            empty_node_deref.size += 1;

            node_deref.keys[0] = sibling_node_deref.keys[0].take();
            node_deref.elements[0] = sibling_node_deref.elements[0].take();
            sibling_node_deref.remove_element_soft(0);

            return
        } else if index == node_size 
            && second_sibling_node.as_ref().unwrap().read().unwrap().size > 1 {
            let mut sibling_node_deref = second_sibling_node.as_ref().unwrap().write().unwrap();
            let sibling_size = sibling_node_deref.size;
            
            empty_node_deref.keys[0] = node_deref.keys[node_size - 1].take();
            empty_node_deref.elements[0] = node_deref.elements[node_size - 1].take();
            empty_node_deref.size += 1;

            node_deref.keys[node_size - 1] = sibling_node_deref.keys[sibling_size - 1].take();
            node_deref.elements[node_size - 1] 
                = sibling_node_deref.elements[sibling_size - 1].take();
            sibling_node_deref.size -= 1;

            return
        } else {
            let l_sibling = if index != 0 {
                node_deref.children[index - 1].clone()
            } else { None };
            let r_sibling = if index + 1 <= node_size {
                node_deref.children[index + 1].clone()
            } else { None };
            if l_sibling.is_some() && l_sibling.as_ref().unwrap().read().unwrap().size > 1 {
                let mut l_sibling_deref = l_sibling.as_ref().unwrap().write().unwrap();
                let l_sibling_size = l_sibling_deref.size;

                empty_node_deref.keys[0] = node_deref.keys[index - 1].take();
                empty_node_deref.elements[0] = node_deref.elements[index - 1].take();
                empty_node_deref.size += 1;

                node_deref.keys[index - 1] = l_sibling_deref.keys[l_sibling_size - 1].take();
                node_deref.elements[index - 1] 
                    = l_sibling_deref.elements[l_sibling_size - 1].take();
                l_sibling_deref.remove_element_soft(l_sibling_size - 1);

                return
            } else if r_sibling.is_some() && r_sibling.as_ref().unwrap().read().unwrap().size > 1 {
                let mut r_sibling_deref = r_sibling.as_ref().unwrap().write().unwrap();

                empty_node_deref.keys[0] = node_deref.keys[index].take();
                empty_node_deref.elements[0] = node_deref.elements[index].take();
                empty_node_deref.size += 1;

                node_deref.keys[index] = r_sibling_deref.keys[0].take();
                node_deref.elements[index] = r_sibling_deref.elements[0].take();
                r_sibling_deref.remove_element_soft(0);

                return
            // both siblings contain one element
            } else {
                // 7. If the parent of the empty leaf stores more than one element, 
                //    move the nearest parent element to the closest
                //    sibling of the empty leaf, remove the empty leaf, 
                //    and finish this operation.
                if node_size >= 2 {
                    if index == 0 {
                        let mut r_sibling_deref = r_sibling.as_ref().unwrap().write().unwrap();
                        
                        r_sibling_deref.shift_right(0);
                        r_sibling_deref.keys[0] = node_deref.keys[index].take();
                        r_sibling_deref.elements[0] = node_deref.elements[index].take();
                        r_sibling_deref.size += 1;
                        node_deref.remove_element_soft(index);
                        node_deref.children[index] = None;
                        node_deref.shift_left_children(0);
                        
                        return
                    } else {
                        let mut l_sibling_deref = l_sibling.as_ref().unwrap().write().unwrap();
                        let l_sibling_size = l_sibling_deref.size;
                        
                        l_sibling_deref.keys[l_sibling_size] = node_deref.keys[index - 1].take();
                        l_sibling_deref.elements[l_sibling_size] = node_deref.elements[index - 1].take();
                        l_sibling_deref.size += 1;
                        node_deref.remove_element_soft(index - 1);
                        node_deref.children[index] = None;
                        node_deref.shift_left_children(index);
                        
                        return
                    }
                } else {
                    // 8. Because both the parent of the empty leaf and the only sibling 
                    //    store only single elements, move the element from this sibling 
                    //    to the parent, remove both children of this parent,
                    //    and continue in step 9 to rebalance the tree because 
                    //    leaves are not at the same level.
                    if index == 0 {
                        let mut r_sibling_deref = r_sibling.as_ref().unwrap().write().unwrap();
                        node_deref.shift_right(1);
                        node_deref.keys[1] = r_sibling_deref.keys[0].take();
                        node_deref.elements[1] = r_sibling_deref.elements[0].take();
                        node_deref.size += 1;
                        node_deref.is_leaf = true;
                        node_deref.children[0] = None;
                        node_deref.children[1] = None;
                    } else {
                        let mut l_sibling_deref = l_sibling.as_ref().unwrap().write().unwrap();
                        node_deref.shift_right(index - 1);
                        node_deref.keys[index - 1] = l_sibling_deref.keys[0].take();
                        node_deref.elements[index - 1] = l_sibling_deref.elements[0].take();
                        node_deref.size += 1;
                        node_deref.is_leaf = true;
                        node_deref.children[index - 1] = None;
                        node_deref.children[index] = None;
                    }
                    // 9. Continue in step 9 to rebalance the tree because leaves 
                    //    are not at the same level. Next finish
                    if !Arc::ptr_eq(&node, &self.root) {
                        self.rebalance(
                            node.clone(),
                            &mut node_deref,
                            parent.unwrap(),
                            parent_index.unwrap()
                        );
                    }
                }
            }
        }
    }

    pub fn rebalance(
        &mut self, 
        node: Arc<RwLock<Node<Key, ORDER>>>, 
        mut node_deref: &mut Node<Key, ORDER>, 
        parent: Arc<RwLock<Node<Key, ORDER>>>,
        parent_index: usize
    ) {
        // 9. If one of the sibling node of the reduced subtree root contains 
        //    more than one element, shift it closest element to the parent 
        //    replacing the closest element in it, create a new child
        //    and move down the replaced element from t he parent to this new child. 
        //    Connect this new child to the reduced subtree and switch the closest child
        //    of the sibling to this new child as well. Next, finish
        // let mut node_deref = node.write().unwrap();
        let mut parent_deref = parent.write().unwrap();

        let l_sibling = if parent_index != 0 {
            parent_deref.children[parent_index - 1].clone()
        } else { None };
        let r_sibling = if parent_index + 1 <= parent_deref.size {
            parent_deref.children[parent_index + 1].clone()
        } else { None };
        
        if l_sibling.is_some() && l_sibling.as_ref().unwrap().read().unwrap().size > 1 {
            let mut l_sibling_deref = l_sibling.as_ref().unwrap().write().unwrap();
            let l_sibling_size = l_sibling_deref.size;

            let new_node = Arc::new(RwLock::new(Node::<Key, ORDER>::new(false, None)));
            let mut new_node_deref = new_node.write().unwrap();
            new_node_deref.keys[0] = parent_deref.keys[parent_index - 1].take();
            new_node_deref.elements[0] = parent_deref.elements[parent_index - 1].take();
            new_node_deref.size += 1;
            new_node_deref.children[0] = l_sibling_deref.children[l_sibling_size].take();
            new_node_deref.children[1] = Some(node.clone());

            new_node_deref.parent = Some(Arc::downgrade(&parent));
            node_deref.parent = Some(Arc::downgrade(&new_node));
            new_node_deref.children[0].as_mut().unwrap().write().unwrap().parent 
                = Some(Arc::downgrade(&new_node));

            parent_deref.children[parent_index] = Some(new_node.clone());
            parent_deref.keys[parent_index - 1] 
                = l_sibling_deref.keys[l_sibling_size - 1].take();
            parent_deref.elements[parent_index - 1] 
                = l_sibling_deref.elements[l_sibling_size - 1].take();
            l_sibling_deref.size -= 1;

            return
        } else if r_sibling.is_some() && r_sibling.as_ref().unwrap().read().unwrap().size > 1 {
            let mut r_sibling_deref = r_sibling.as_ref().unwrap().write().unwrap();

            let new_node = Arc::new(RwLock::new(Node::<Key, ORDER>::new(false, None)));
            let mut new_node_deref = new_node.write().unwrap();
            new_node_deref.keys[0] = parent_deref.keys[parent_index].take();
            new_node_deref.elements[0] = parent_deref.elements[parent_index].take();
            new_node_deref.size += 1;
            new_node_deref.children[0] = Some(node.clone());
            new_node_deref.children[1] = r_sibling_deref.children[0].take();

            new_node_deref.parent = Some(Arc::downgrade(&parent));
            node_deref.parent = Some(Arc::downgrade(&new_node));
            new_node_deref.children[1].as_mut().unwrap().write().unwrap().parent 
                = Some(Arc::downgrade(&new_node));

            parent_deref.children[parent_index] = Some(new_node.clone());
            parent_deref.keys[parent_index] = r_sibling_deref.keys[0].take();
            parent_deref.elements[parent_index] = r_sibling_deref.elements[0].take();
            r_sibling_deref.remove_element_soft(0);
            r_sibling_deref.shift_left_children(0);

            return
        }
        // 10. Move the parent element to the closest one element sibling of the reduced
        //     subtree root and switch this reduced subtree root from its parent to this sibling.
        //     Next, finish this operation if the parent node still contains one element.
        //     If not, start the rebalancing operation for this sib ling going to step 9 until the
        //     main root of the tree is not achieved.
        if let Some(l_sibling) = l_sibling {
            let mut l_sibling_deref = l_sibling.write().unwrap();
            let mut l_sibling_size = l_sibling_deref.size;

            l_sibling_deref.keys[l_sibling_size] = parent_deref.keys[parent_index - 1].take();
            l_sibling_deref.elements[l_sibling_size] 
                = parent_deref.elements[parent_index - 1].take();
            l_sibling_deref.size += 1;
            l_sibling_size += 1;
            l_sibling_deref.children[l_sibling_size] = Some(node.clone());
            parent_deref.remove_element_soft(parent_index - 1);
            parent_deref.shift_left_children(parent_index);
            node_deref.parent = Some(Arc::downgrade(&l_sibling));
        } else if let Some(r_sibling) = r_sibling {
            let mut r_sibling_deref = r_sibling.write().unwrap();

            r_sibling_deref.shift_right(0);
            r_sibling_deref.keys[0] = parent_deref.keys[parent_index].take();
            r_sibling_deref.elements[0] = parent_deref.elements[parent_index].take();
            r_sibling_deref.size += 1;
            r_sibling_deref.shift_right_children(0);
            r_sibling_deref.children[0] = Some(node.clone());
            parent_deref.remove_element_soft(parent_index);
            parent_deref.shift_left_children(parent_index);
            node_deref.parent = Some(Arc::downgrade(&r_sibling));
        }
        if parent_deref.size >= 1 {
            return
        } else if Arc::ptr_eq(&parent, &self.root) {
            self.root = parent_deref.children[0].as_ref().unwrap().clone();
            self.root.write().unwrap().parent = None;
            return
        } else {
            let parent_of_parent = parent_deref.parent.clone();
            let reduced_subree_root = parent_deref.children[0].as_ref().unwrap().clone();
            let mut reduced_subree_root_deref = reduced_subree_root.write().unwrap();

            if let Some(parent_of_parent_weak) = &parent_of_parent {
                let parent_of_parent = parent_of_parent_weak.upgrade().unwrap();

                let parent_of_parent_index = parent_of_parent.read().unwrap().find_child(&parent);
                parent_of_parent.write().unwrap().children[parent_of_parent_index.unwrap()] 
                    = Some(reduced_subree_root.clone());
                
                reduced_subree_root_deref.parent = Some(parent_of_parent_weak.clone());
                self.rebalance(
                    reduced_subree_root.clone(),
                    &mut reduced_subree_root_deref,
                    parent_of_parent,
                    parent_of_parent_index.unwrap()
                );

                return
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

    pub fn test_graph(&self, print_if_ok: bool) -> bool {
        let mut height = 0;
        let mut node = self.root.clone();
        let mut queue: Vec<Vec<Arc<RwLock<Node<Key, ORDER>>>>> = vec![vec![]];
        queue[0].push(node.clone());

        let mut is_ok = true;

        loop {
            queue.push(vec![]);
            for i in 0..(queue[height].len()) {
                node = queue[height][i].clone();
                let node_size = node.read().unwrap().size;
                for j in 0..(node_size) {
                    if !node.read().unwrap().is_leaf {
                        queue[height + 1].push(node.read().unwrap().children[j].as_ref().unwrap().clone());
                    }
                }
                if !node.read().unwrap().is_leaf {
                    let node_deref = node.read().unwrap();
                    match node_deref.children[node_size].as_ref() {
                        Some(child) => queue[height + 1].push(child.clone()),
                        None => { 
                            print!("something went wrong with "); 
                            node_deref.print_node(false);
                        }
                    };
                }
            }
            if queue.last().unwrap().len() > 0 {
                height += 1;
            } else {
                for level in &queue {
                    for node in level {
                        if !node.read().unwrap().test_node(true) { is_ok = false; }
                    }
                }
                if !is_ok {
                    println!("asa-graph is not ok:");
                    self.print_graph() 
                } else if print_if_ok {
                    self.print_graph() 
                }
                io::stdout().flush().unwrap();

                return is_ok
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
        &mut self,
        node: &Arc<RwLock<Node<Key, ORDER>>>,
        key: &Key,
        weighting_strategy: Arc<dyn DefiningWeightingStrategyAsync>,
        interelement_activation_threshold: f32,
        interelement_activation_exponent: i32
    ) -> Arc<RwLock<Element<Key, ORDER>>> {
        let element_pointer = Element::<Key, ORDER>::new_custom(
            key, 
            1, 
            self.id, 
            weighting_strategy,
            interelement_activation_threshold,
            interelement_activation_exponent
        );
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
                    DataCategory::Continuous | DataCategory::Discrete => {
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

impl<'a, Key, const ORDER: usize> Clone for ASAGraph<Key, ORDER> 
where Key: SensorData + Sync + Send, [(); ORDER + 1]: {
    fn clone(&self) -> Self {
        ASAGraph {
            id: self.id,
            root: self.root.clone(),
            element_min: self.element_min.clone(),
            element_max: self.element_max.clone(),
            elements_counter: self.elements_counter,
            key_min: if let Some(key) = &self.key_min { 
                Some(*dyn_clone::clone_box(key)) 
            } else { None },
            key_max: if let Some(key) = &self.key_max { 
                Some(*dyn_clone::clone_box(key)) 
            } else { None },
            data_type: self.data_type
        }
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
    use std::{ time::Instant };
    
    use rand::{
        Rng, 
        seq::SliceRandom,
        rngs::StdRng,
        SeedableRng
    };

    use witchnet_common::{
        neuron::NeuronAsync,
        data::DataTypeValue
    };

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
    fn fuzzy_search() {
        let mut graph = ASAGraph::<i32, 3>::new(1);
        for i in [1, 2, 3, 5, 6, 7] {
            graph.insert(&i);
        }
        let result = graph.fuzzy_search(&4, 0.8, false);
        assert!(result.is_some());
        assert_eq!(result.unwrap().0.read().unwrap().key, 3);
        assert!(graph.fuzzy_search(&10, 0.8, false).is_none());
        assert!(graph.fuzzy_search(&-2, 0.8, false).is_none());

        let mut graph = ASAGraph::<f64, 3>::new(1);
        for i in [1.0, 2.0, 3.0, 5.0, 6.0, 7.0] {
            graph.insert(&i);
        }
        let result = graph.fuzzy_search(&3.1, 0.95, false);
        assert!(result.is_some());
        assert_eq!(result.unwrap().0.read().unwrap().key, 3.0);
        assert!(graph.fuzzy_search(&3.5, 0.95, false).is_none());
        assert!(graph.fuzzy_search(&3.5, 0.95, true).is_some());
        assert!(graph.fuzzy_search(&-1.0, 0.95, false).is_none());

        let mut graph = ASAGraph::<String, 3>::new(1);
        for i in ["1".to_string(), "2".to_string()] {
            graph.insert(&i);
        }
        let result = graph.fuzzy_search(&"1".to_string(), 0.8, false);
        assert!(result.is_some());
        assert_eq!(result.unwrap().0.read().unwrap().key, "1".to_string());
        assert!(graph.fuzzy_search(&"3".to_string(), 0.8, false).is_none());
        assert!(graph.fuzzy_search(&"3".to_string(), 0.8, true).is_none());
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

    #[test]
    fn levels() {
        let mut graph = ASAGraph::<i32, 3>::new(1);
        for i in 1..=25 { graph.insert(&i); }
        let lvlvs: Vec<Vec<Vec<DataTypeValue>>> = graph.levels().into_iter().map(
            |v| v.into_iter().map(
                |n| n.into_iter().map(
                    |e| {
                        e.read().unwrap().value()
                    }
                ).collect()
            ).collect()
        ).collect();
        println!("{:?}", lvlvs);
    }

    #[test]
    fn remove() {
        let mut rng = StdRng::seed_from_u64(42);

        let mut graph = ASAGraph::<i32, 5>::new(1);

        let mut numbers = vec![];
        for _ in 0..5150 {
            let number: i32 = rng.gen_range(1..=2850);
            numbers.push(number);
            graph.insert(&number);
        }
        numbers.shuffle(&mut rng);

        graph.print_graph();

        print!("removing ");
        for number in &numbers {
            print!("{number} ");
            graph.remove(number);
            if !graph.test_graph(false) {
                println!("removing {number} went wrong, returning");
                return
            }
        }
        println!();

        graph.print_graph();
    }
}