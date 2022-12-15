use std::{
    fmt::{ Display, Formatter, Result },
    sync::{ Arc, Weak, RwLock },
    cmp::Ordering::*,
    marker::PhantomData
};

use witchnet_common::{
    algorithms::SearchAlgorithm,
    sensor::SensorData,
    data::{ DataDeductor, DataTypeValue }, 
    connection::collective::defining::DefiningWeightingStrategyAsync
};

use super::element::Element;

#[derive(Clone)]
pub struct Node<Key, const ORDER: usize>
where Key: SensorData, [(); ORDER + 1]: {
    pub(crate) size: usize,
    pub(crate) is_leaf: bool,
    pub(crate) parent: Option<Weak<RwLock<Node<Key, ORDER>>>>,
    pub(crate) keys: [Option<Key>; ORDER],
    pub(crate) elements: [Option<Arc<RwLock<Element<Key, ORDER>>>>; ORDER],
    pub(crate) children: [Option<Arc<RwLock<Node<Key, ORDER>>>>; ORDER + 1]
}

impl<Key, const ORDER: usize> Node<Key, ORDER> 
where 
    Key: SensorData + Sync + Send, 
    [(); ORDER + 1]:, 
    PhantomData<Key>: DataDeductor,
    DataTypeValue: From<Key>
{
    pub fn new(
        is_leaf: bool, parent: Option<Weak<RwLock<Node<Key, ORDER>>>>
    ) -> Node<Key, ORDER> {
        Node {
            size: 0,
            is_leaf,
            parent,
            keys: [(); ORDER].map(|_| None),
            elements: [(); ORDER].map(|_| None),
            children: [(); ORDER + 1].map(|_| None)
        }
    }

    pub fn find_element(&self, key: &Key, algorithm: SearchAlgorithm)
    -> Option<Arc<RwLock<Element<Key, ORDER>>>> {
        match algorithm {
            SearchAlgorithm::LeftSearch => {
                for i in 0..self.size {
                    if self.keys[i].as_ref().unwrap().equals(key) { 
                        return Some(self.elements[i].as_ref().unwrap().clone())
                    }
                }
                return None
            },
            SearchAlgorithm::RightSearch => {
                for i in (0..self.size).rev() {
                    if self.keys[i].as_ref().unwrap().equals(key) { 
                        return Some(self.elements[i].as_ref().unwrap().clone())
                    }
                }
                return None
            },
            SearchAlgorithm::BinarySearch => {
                let mut left_index = 0;
                let mut right_index = self.size - 1; 
                while left_index <= right_index {
                    let mid_index = left_index + (right_index - left_index) / 2;
                    let mid_key = self.keys[mid_index].as_ref().unwrap();
                    if mid_key.partial_compare(key) == Some(Less) {
                        left_index = mid_index + 1;
                    } else if mid_key.partial_compare(key) == Some(Greater) {
                        right_index = mid_index - 1;
                    } else {
                        return Some(self.elements[mid_index].as_ref().unwrap().clone())
                    }
                }
                return None
            }
        }
    }

    pub fn split_child(node_ptr: &Arc<RwLock<Node<Key, ORDER>>>, child_index: usize) {
        let node_size = node_ptr.write().unwrap().size;

        let left_node = 
            node_ptr.read().unwrap().children[child_index].as_ref().unwrap().clone();
        left_node.write().unwrap().parent = Some(Arc::downgrade(node_ptr));

        let mut right_node = Node::<Key, ORDER>::new(
            left_node.read().unwrap().is_leaf, Some(Arc::downgrade(node_ptr))
        );
        right_node.parent = Some(Arc::downgrade(node_ptr));
        
        right_node.size = Self::MIN_ELEMENTS;
        left_node.write().unwrap().size = Self::MIN_ELEMENTS;

        for i in 0..Self::MIN_ELEMENTS {
            right_node.elements[i] = left_node.write().unwrap().elements[Self::T_OFFSET + i].take();
            right_node.keys[i] = left_node.write().unwrap().keys[Self::T_OFFSET + i].take();
        }

        for i in ((child_index + 1)..=(node_size)).rev() {
            node_ptr.write().unwrap().children.swap(i, i + 1);
        }
        for i in ((child_index as isize)..=(node_size as isize - 1isize)).rev() {
            node_ptr.write().unwrap().elements.swap(i as usize, (i + 1) as usize);
            node_ptr.write().unwrap().keys.swap(i as usize, (i + 1) as usize);
        }

        node_ptr.write().unwrap().children[child_index + 1] = Some(Arc::new(RwLock::new(right_node)));
        let right_node = 
            node_ptr.read().unwrap().children[child_index + 1].as_ref().unwrap().clone();
        
        if !left_node.read().unwrap().is_leaf {
            for i in 0..Self::MIN_CHILDREN {
                right_node.write().unwrap().children[i] = 
                    left_node.write().unwrap().children[Self::T_OFFSET + i].take();
                right_node.write().unwrap().children[i].as_ref().unwrap().write().unwrap().parent 
                    = Some(Arc::downgrade(&right_node));
            }
        }

        node_ptr.write().unwrap().elements[child_index] = 
            left_node.write().unwrap().elements[Self::MID_INDEX].take();
        node_ptr.write().unwrap().keys[child_index] = 
            left_node.write().unwrap().keys[Self::MID_INDEX].take();
        node_ptr.write().unwrap().size += 1;
    }

    pub(crate) fn insert_existing_key(&self, key: &Key, left_search: bool)
    -> (Option<Arc<RwLock<Element<Key, ORDER>>>>, usize) {
        let mut index;
        if left_search {
            index = 0usize;
            let mut current_key = self.keys[index].as_ref().unwrap();
            while index < self.size - 1 && key.partial_compare(current_key) == Some(Greater) {
                index += 1;
                current_key = self.keys[index].as_ref().unwrap();
            }
            if key.partial_compare(current_key) == Some(Greater) {
                index += 1;
            }
            if index < self.size && key.equals(current_key) {
                let element = self.elements[index].as_ref().unwrap().clone();
                element.write().unwrap().counter += 1;
                return (Some(element), index)
            }
        } else {
            index = self.size - 1;
            let mut current_key = self.keys[index].as_ref().unwrap();
            while index > 0 && key.partial_compare(current_key) == Some(Less)  {
                index -= 1;
                current_key = self.keys[index].as_ref().unwrap();
            }
            if key.partial_compare(current_key) == Some(Greater) {
                index += 1;
            } else if key.equals(current_key) {
                let element = self.elements[index].as_ref().unwrap().clone();
                element.write().unwrap().counter += 1;
                return (Some(element), index)
            }

            if index < self.size && key.equals(current_key) {
                let element = self.elements[index].as_ref().unwrap().clone();
                element.write().unwrap().counter += 1;
                return (Some(element), index)
            }
        }
        (None, index)
    }

    pub(crate) fn insert_key_leaf(
        node: &Arc<RwLock<Node<Key, ORDER>>>, 
        key: &Key, 
        element_id: u32,
        parent_id: u32,
        range: f32,
        weighting_strategy: Arc<dyn DefiningWeightingStrategyAsync>,
        interelement_activation_threshold: f32,
        interelement_activation_exponent: i32
    ) -> Arc<RwLock<Element<Key, ORDER>>> {
        let node_size = node.read().unwrap().size;

        let mut index = node_size - 1;
        let mut counter = node_size as isize - 1;
        let mut should_move = false;
        while counter >= 0 && key.partial_compare(node.read().unwrap().keys[counter as usize].as_ref().unwrap()) == Some(Less) {
            should_move = true;
            index = counter as usize;
            counter -= 1;
        }

        if should_move {
            for i in (index..node_size).rev() {
                node.write().unwrap().elements.swap(i, i + 1);
                node.write().unwrap().keys.swap(i, i + 1);
            }
        } else {
            index += 1;
        }
        
        let new_element = Element::new_custom(
            key, 
            element_id, 
            parent_id, 
            weighting_strategy,
            interelement_activation_threshold,
            interelement_activation_exponent
        );
        node.write().unwrap().elements[index] = Some(new_element.clone());
        node.write().unwrap().keys[index] = Some(*dyn_clone::clone_box(key));

        let mut next_ptr = None;
        let mut prev_ptr = None;
        if node_size >= 1 {
            if index == 0 {
                if let Some(next) = node.read().unwrap().elements[1].as_ref() {
                    next_ptr = Some(next.clone());
                    prev_ptr = match next.as_ref().read().unwrap().prev.as_ref() {
                        Some(v) => Some(v.0.upgrade().unwrap()),
                        None => None
                    };
                }
            } else {
                if let Some(prev) = node.read().unwrap().elements[index - 1].as_ref() {
                    prev_ptr = Some(prev.clone());
                    next_ptr = match prev.as_ref().read().unwrap().next.as_ref() {
                        Some(e) => Some(e.0.upgrade().unwrap().clone()),
                        None => None
                    };
                }
            }
        }

        Element::<Key, ORDER>::set_connections(
            &new_element, prev_ptr.as_ref(), next_ptr.as_ref(), range
        );
        node.write().unwrap().size += 1;

        new_element
    }

    pub const MIN_CHILDREN: usize = (ORDER + 1) / 2;
    pub const MAX_CHILDREN: usize = ORDER + 1;
    pub const MIN_ELEMENTS: usize = (ORDER + 1) / 2 - 1;
    pub const MAX_ELEMENTS: usize = ORDER;
    pub const MIN_KEYS: usize = (ORDER + 1) / 2 - 1;
    pub const MAX_KEYS: usize = ORDER;
    pub(crate) const MID_INDEX: usize = (ORDER + 1) / 2 - 1;
    pub(crate) const T_OFFSET: usize = (ORDER + 1) / 2;
}

impl<Key, const ORDER: usize> Display for Node<Key, ORDER> 
where Key: SensorData + Sync + Send, [(); ORDER + 1]: {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut node: String = format!(
            "<==leaf:{}, size:{}, keys:", self.is_leaf, self.size
        );
        for key in &self.keys {
            let element: String = match key.as_ref() {
                Some(k) => format!("({k})"),
                None => "()".to_string()
            };
            node.push_str(&element);
        }
        node.push_str(", elements:");
        for i in 0..self.elements.len() {
            let element: String = match self.elements[i].as_ref() {
                Some(e) => format!("{}", e.as_ref().read().unwrap()),
                None => "[]".to_string()
            };
            node.push_str(&element);
        }
        node.push_str(", children:");
        for child in &self.children {
            let child_node: String = match child.as_ref() {
                Some(n) => format!("{}", n.as_ref().read().unwrap()),
                None => "<>".to_string()
            };
            node.push_str(&child_node);
        }
        node.push_str("==>");
        write!(f, "{}", node)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{ Arc, RwLock };

    use witchnet_common::connection::collective::defining::ConstantOneWeightAsync;

    use super::super::{
        node::Node,
        element::Element,
        graph::ASAGraph
    };

    #[test]
    fn print_empty_node() {
        type NodeTest = Node::<i32, 3>;
        let node = NodeTest::new(true, None);
        println!("empty node: {}", node);
    }

    #[test]
    fn insert_into_leaf() {
        let graph = Arc::new(RwLock::new(ASAGraph::<i32, 3>::new(1)));
        let root: &Arc<RwLock<Node<i32, 3>>> = &graph.read().unwrap().root;
        let graph_id = graph.read().unwrap().id;

        root.write().unwrap().elements[0] = Some(Element::new(&2, 1, graph_id));
        root.write().unwrap().keys[0] = Some(2);
        root.write().unwrap().size = 1;

        let weighting_strategy = Arc::new(ConstantOneWeightAsync);
        Node::insert_key_leaf(&root, &-1, 2, graph_id, 1f32, weighting_strategy.clone(), 0.00001, 1);
        Node::insert_key_leaf(&root, &1, 3, graph_id, 1f32, weighting_strategy.clone(), 0.00001, 1);
        root.read().unwrap().insert_existing_key(&1, true);
        root.read().unwrap().insert_existing_key(&-1, true);
        root.read().unwrap().insert_existing_key(&2, true);
        root.read().unwrap().insert_existing_key(&1, false);
        root.read().unwrap().insert_existing_key(&-1, false);
        root.read().unwrap().insert_existing_key(&2, false);
        root.read().unwrap().insert_existing_key(&3, false);
       
        assert!(root.read().unwrap().size == 3);
        assert_eq!(root.read().unwrap().keys[0].unwrap(), -1);
        assert_eq!(root.read().unwrap().keys[1].unwrap(), 1);
        assert_eq!(root.read().unwrap().keys[2].unwrap(), 2);
        assert_eq!(root.read().unwrap().elements[0].as_ref().unwrap().read().unwrap().counter, 3);
        assert_eq!(root.read().unwrap().elements[1].as_ref().unwrap().read().unwrap().counter, 3);
        assert_eq!(root.read().unwrap().elements[2].as_ref().unwrap().read().unwrap().counter, 3);
    }

    #[test]
    fn split_node() {
        let graph = Arc::new(RwLock::new(ASAGraph::<i32, 3>::new(1)));
        let root: &Arc<RwLock<Node<i32, 3>>> = &graph.read().unwrap().root;
        let graph_id = graph.read().unwrap().id;

        root.write().unwrap().elements[0] = Some(Element::new(&1, 1, graph_id));
        root.write().unwrap().keys[0] = Some(1);
        root.write().unwrap().size = 1;

        let weighting_strategy = Arc::new(ConstantOneWeightAsync);
        Node::insert_key_leaf(&root, &6, 2, graph_id, 5f32, weighting_strategy.clone(), 0.00001, 1);
        Node::insert_key_leaf(&root, &7, 3, graph_id, 6f32, weighting_strategy.clone(), 0.00001, 1);

        let root_new = Arc::new(RwLock::new(Node::new(false, None)));
        root_new.write().unwrap().children[0] = Some(root.clone());
        root.write().unwrap().parent = Some(Arc::downgrade(&root_new));
        Node::split_child(&root_new, 0);

        assert_eq!(root_new.read().unwrap().size, 1);
        assert_eq!(root_new.read().unwrap().children[0].as_ref().unwrap().read().unwrap().size, 1);
        assert_eq!(root_new.read().unwrap().children[1].as_ref().unwrap().read().unwrap().size, 1);
        assert!(root_new.read().unwrap().children[2].is_none());
        assert!(root_new.read().unwrap().children[0].as_ref().unwrap().read().unwrap().children[0].is_none());
        assert!(root_new.read().unwrap().children[1].as_ref().unwrap().read().unwrap().children[0].is_none());

        assert_eq!(root_new.read().unwrap().keys[0].unwrap(), 6);
        assert_eq!(root_new.read().unwrap().children[0].as_ref().unwrap().read().unwrap().keys[0].unwrap(), 1);
        assert_eq!(root_new.read().unwrap().children[1].as_ref().unwrap().read().unwrap().keys[0].unwrap(), 7);
        assert!(root_new.read().unwrap().keys[1].is_none());
        assert!(root_new.read().unwrap().children[0].as_ref().unwrap().read().unwrap().keys[1].is_none());
        assert!(root_new.read().unwrap().children[1].as_ref().unwrap().read().unwrap().keys[1].is_none());
        
        assert_eq!(root_new.read().unwrap().elements[0].as_ref().unwrap().read().unwrap().key, 6);
        assert_eq!(root_new.read().unwrap().children[0].as_ref().unwrap().read().unwrap().elements[0].as_ref().unwrap().read().unwrap().key, 1);
        assert_eq!(root_new.read().unwrap().children[1].as_ref().unwrap().read().unwrap().elements[0].as_ref().unwrap().read().unwrap().key, 7);
        assert!(root_new.read().unwrap().elements[1].is_none());
        assert!(root_new.read().unwrap().children[0].as_ref().unwrap().read().unwrap().elements[1].is_none());
        assert!(root_new.read().unwrap().children[1].as_ref().unwrap().read().unwrap().elements[1].is_none());

        Node::insert_key_leaf(&root_new, &2, 4, graph_id, 6f32, weighting_strategy.clone(), 0.00001, 1);
        Node::insert_key_leaf(&root_new, &4, 5, graph_id, 6f32, weighting_strategy.clone(), 0.00001, 1);

        let middle_left_node = Arc::new(
            RwLock::new(Node::new(true, Some(Arc::downgrade(&root_new))))
        );
        middle_left_node.write().unwrap().elements[0] = Some(Element::new(&3, 6, graph_id));
        middle_left_node.write().unwrap().keys[0] = Some(3);
        middle_left_node.write().unwrap().size = 1;

        let middle_right_node = Arc::new(
            RwLock::new(Node::new(true, Some(Arc::downgrade(&root_new))))
        );
        middle_right_node.write().unwrap().elements[0] = Some(Element::new(&5, 7, graph_id));
        middle_right_node.write().unwrap().keys[0] = Some(5);
        middle_right_node.write().unwrap().size = 1;

        root_new.write().unwrap().children.swap(1, 3);
        root_new.write().unwrap().children[1] = Some(middle_left_node);
        root_new.write().unwrap().children[2] = Some(middle_right_node);

        let root_newer = Arc::new(RwLock::new(Node::new(false, None)));
        root_newer.write().unwrap().children[0] = Some(root_new.clone());
        root_new.write().unwrap().parent = Some(Arc::downgrade(&root_newer));
        Node::split_child(&root_newer, 0);

        let left_child = root_newer.read().unwrap().children[0].as_ref().unwrap().clone();
        let right_child = root_newer.read().unwrap().children[1].as_ref().unwrap().clone();
        
        let left_left_child = left_child.read().unwrap().children[0].as_ref().unwrap().clone();
        let left_right_child = left_child.read().unwrap().children[1].as_ref().unwrap().clone();
        let right_left_child = right_child.read().unwrap().children[0].as_ref().unwrap().clone();
        let right_right_child = right_child.read().unwrap().children[1].as_ref().unwrap().clone();

        assert_eq!(root_newer.read().unwrap().size, 1);
        assert_eq!(left_child.read().unwrap().size, 1);
        assert_eq!(right_child.read().unwrap().size, 1);
        assert_eq!(left_left_child.read().unwrap().size, 1);
        assert_eq!(left_right_child.read().unwrap().size, 1);
        assert_eq!(right_left_child.read().unwrap().size, 1);
        assert_eq!(right_right_child.read().unwrap().size, 1);
        assert!(root_newer.read().unwrap().children[2].is_none());
        assert!(left_child.read().unwrap().children[2].is_none());
        assert!(right_child.read().unwrap().children[2].is_none());
        assert!(left_left_child.read().unwrap().children[0].is_none());
        assert!(left_right_child.read().unwrap().children[0].is_none());
        assert!(right_left_child.read().unwrap().children[0].is_none());
        assert!(right_right_child.read().unwrap().children[0].is_none());
        assert_eq!(root_newer.read().unwrap().is_leaf, false);
        assert_eq!(left_child.read().unwrap().is_leaf, false);
        assert_eq!(right_child.read().unwrap().is_leaf, false);
        assert_eq!(left_left_child.read().unwrap().is_leaf, true);
        assert_eq!(left_right_child.read().unwrap().is_leaf, true);
        assert_eq!(right_left_child.read().unwrap().is_leaf, true);
        assert_eq!(right_right_child.read().unwrap().is_leaf, true);
        
        assert!(root_newer.read().unwrap().keys[1].is_none());
        assert!(left_child.read().unwrap().keys[1].is_none());
        assert!(right_child.read().unwrap().keys[1].is_none());
        assert!(left_left_child.read().unwrap().keys[1].is_none());
        assert!(left_right_child.read().unwrap().keys[1].is_none());
        assert!(right_left_child.read().unwrap().keys[1].is_none());
        assert!(right_right_child.read().unwrap().keys[1].is_none());
        
        assert!(root_newer.read().unwrap().elements[1].is_none());
        assert!(left_child.read().unwrap().elements[1].is_none());
        assert!(right_child.read().unwrap().elements[1].is_none());
        assert!(left_left_child.read().unwrap().elements[1].is_none());
        assert!(left_right_child.read().unwrap().elements[1].is_none());
        assert!(right_left_child.read().unwrap().elements[1].is_none());
        assert!(right_right_child.read().unwrap().elements[1].is_none());

        assert_eq!(root_newer.read().unwrap().keys[0].unwrap(), 4);
        assert_eq!(left_child.read().unwrap().keys[0].unwrap(), 2);
        assert_eq!(right_child.read().unwrap().keys[0].unwrap(), 6);
        assert_eq!(left_left_child.read().unwrap().keys[0].unwrap(), 1);
        assert_eq!(left_right_child.read().unwrap().keys[0].unwrap(), 3);
        assert_eq!(right_left_child.read().unwrap().keys[0].unwrap(), 5);
        assert_eq!(right_right_child.read().unwrap().keys[0].unwrap(), 7);

        assert_eq!(root_newer.read().unwrap().elements[0].as_ref().unwrap().read().unwrap().key, 4);
        assert_eq!(left_child.read().unwrap().elements[0].as_ref().unwrap().read().unwrap().key, 2);
        assert_eq!(right_child.read().unwrap().elements[0].as_ref().unwrap().read().unwrap().key, 6);
        assert_eq!(left_left_child.read().unwrap().elements[0].as_ref().unwrap().read().unwrap().key, 1);
        assert_eq!(left_right_child.read().unwrap().elements[0].as_ref().unwrap().read().unwrap().key, 3);
        assert_eq!(right_left_child.read().unwrap().elements[0].as_ref().unwrap().read().unwrap().key, 5);
        assert_eq!(right_right_child.read().unwrap().elements[0].as_ref().unwrap().read().unwrap().key, 7);
    }
}