use std::{
    fmt::{ Display, Formatter, Result },
    rc::{ Rc, Weak },
    cell::{ RefCell, Ref, RefMut },
    cmp::Ordering::*,
    marker::PhantomData
};

use witchnet_common::{
    algorithms::SearchAlgorithm,
    sensor::SensorData,
    data::{ DataDeductor, DataTypeValue }, 
    connection::collective::defining::DefiningWeightingStrategy
};

use super::element::Element;

#[derive(Clone)]
pub struct Node<Key, const ORDER: usize>
where Key: SensorData, [(); ORDER + 1]: {
    pub(crate) size: usize,
    pub(crate) is_leaf: bool,
    pub(crate) parent: Option<Weak<RefCell<Node<Key, ORDER>>>>,
    pub(crate) keys: [Option<Key>; ORDER],
    pub(crate) elements: [Option<Rc<RefCell<Element<Key, ORDER>>>>; ORDER],
    pub(crate) children: [Option<Rc<RefCell<Node<Key, ORDER>>>>; ORDER + 1]
}

impl<Key, const ORDER: usize> Node<Key, ORDER> 
where 
    Key: SensorData, 
    [(); ORDER + 1]:, 
    PhantomData<Key>: DataDeductor,
    DataTypeValue: From<Key>
{
    pub fn new(
        is_leaf: bool, parent: Option<Weak<RefCell<Node<Key, ORDER>>>>
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
    -> Option<Ref<Element<Key, ORDER>>> {
        match algorithm {
            SearchAlgorithm::LeftSearch => {
                for i in 0..self.size {
                    if self.keys[i].as_ref().unwrap().equals(key) { 
                        return Some(self.elements[i].as_ref().unwrap().as_ref().borrow()) 
                    }
                }
                return None
            },
            SearchAlgorithm::RightSearch => {
                for i in (0..self.size).rev() {
                    if self.keys[i].as_ref().unwrap().equals(key) { 
                        return Some(self.elements[i].as_ref().unwrap().as_ref().borrow()) 
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
                        return Some(
                            self.elements[mid_index].as_ref().unwrap().as_ref().borrow()
                        );
                    }
                }
                return None
            }
        }
    }

    pub fn find_element_mut(&self, key: &Key, algorithm: SearchAlgorithm)
    -> Option<RefMut<Element<Key, ORDER>>> {
        match algorithm {
            SearchAlgorithm::LeftSearch => {
                for i in 0..self.size {
                    if self.keys[i].as_ref().unwrap().equals(key) { 
                        return Some(self.elements[i].as_ref().unwrap().as_ref().borrow_mut()) 
                    }
                }
                return None
            },
            SearchAlgorithm::RightSearch => {
                for i in (0..self.size).rev() {
                    if self.keys[i].as_ref().unwrap().equals(key) { 
                        return Some(self.elements[i].as_ref().unwrap().as_ref().borrow_mut()) 
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
                        return Some(
                            self.elements[mid_index].as_ref().unwrap().as_ref().borrow_mut()
                        );
                    }
                }
                return None
            }
        }
    }

    pub fn split_child(node_ptr: &Rc<RefCell<Node<Key, ORDER>>>, child_index: usize) {
        let node_size = node_ptr.borrow_mut().size;

        let left_node = 
            node_ptr.borrow().children[child_index].as_ref().unwrap().clone();
        left_node.borrow_mut().parent = Some(Rc::downgrade(node_ptr));

        let mut right_node = Node::<Key, ORDER>::new(
            left_node.borrow().is_leaf, Some(Rc::downgrade(node_ptr))
        );
        right_node.parent = Some(Rc::downgrade(node_ptr));
        
        right_node.size = Self::MIN_ELEMENTS;
        left_node.borrow_mut().size = Self::MIN_ELEMENTS;

        for i in 0..Self::MIN_ELEMENTS {
            right_node.elements[i] = left_node.borrow_mut().elements[Self::T_OFFSET + i].take();
            right_node.keys[i] = left_node.borrow_mut().keys[Self::T_OFFSET + i].take();
        }

        for i in ((child_index + 1)..=(node_size)).rev() {
            node_ptr.borrow_mut().children.swap(i, i + 1);
        }
        for i in ((child_index as isize)..=(node_size as isize - 1isize)).rev() {
            node_ptr.borrow_mut().elements.swap(i as usize, (i + 1) as usize);
            node_ptr.borrow_mut().keys.swap(i as usize, (i + 1) as usize);
        }

        node_ptr.borrow_mut().children[child_index + 1] = Some(Rc::new(RefCell::new(right_node)));
        let right_node = 
            node_ptr.borrow().children[child_index + 1].as_ref().unwrap().clone();
        
        if !left_node.borrow().is_leaf {
            for i in 0..Self::MIN_CHILDREN {
                right_node.borrow_mut().children[i] = 
                    left_node.borrow_mut().children[Self::T_OFFSET + i].take();
                right_node.borrow_mut().children[i].as_ref().unwrap().borrow_mut().parent 
                    = Some(Rc::downgrade(&right_node));
            }
        }

        node_ptr.borrow_mut().elements[child_index] = 
            left_node.borrow_mut().elements[Self::MID_INDEX].take();
        node_ptr.borrow_mut().keys[child_index] = 
            left_node.borrow_mut().keys[Self::MID_INDEX].take();
        node_ptr.borrow_mut().size += 1;
    }

    pub(crate) fn insert_existing_key(&self, key: &Key, left_search: bool)
    -> (Option<Rc<RefCell<Element<Key, ORDER>>>>, usize) {
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
                element.borrow_mut().counter += 1;
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
                element.borrow_mut().counter += 1;
                return (Some(element), index)
            }

            if index < self.size && key.equals(current_key) {
                let element = self.elements[index].as_ref().unwrap().clone();
                element.borrow_mut().counter += 1;
                return (Some(element), index)
            }
        }
        (None, index)
    }

    pub(crate) fn insert_key_leaf(
        node: &Rc<RefCell<Node<Key, ORDER>>>, 
        key: &Key, 
        element_id: u32,
        parent_id: u32,
        range: f32,
        weighting_strategy: Rc<dyn DefiningWeightingStrategy>,
        interelement_activation_threshold: f32,
        interelement_activation_exponent: i32
    ) -> Rc<RefCell<Element<Key, ORDER>>> {
        let node_size = node.borrow().size;

        let mut index = node_size - 1;
        let mut counter = node_size as isize - 1;
        let mut should_move = false;
        while counter >= 0 && key.partial_compare(
            node.borrow().keys[counter as usize].as_ref().unwrap()
        ) == Some(Less) {
            should_move = true;
            index = counter as usize;
            counter -= 1;
        }

        if should_move {
            for i in (index..node_size).rev() {
                node.borrow_mut().elements.swap(i, i + 1);
                node.borrow_mut().keys.swap(i, i + 1);
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
        node.borrow_mut().elements[index] = Some(new_element.clone());
        node.borrow_mut().keys[index] = Some(*dyn_clone::clone_box(key));

        let mut next_ptr = None;
        let mut prev_ptr = None;
        if node_size >= 1 {
            if index == 0 {
                if let Some(next) = node.borrow().elements[1].as_ref() {
                    next_ptr = Some(next.clone());
                    prev_ptr = match next.as_ref().borrow().prev.as_ref() {
                        Some(v) => Some(v.0.upgrade().unwrap()),
                        None => None
                    };
                }
            } else {
                if let Some(prev) = node.borrow().elements[index - 1].as_ref() {
                    prev_ptr = Some(prev.clone());
                    next_ptr = match prev.as_ref().borrow().next.as_ref() {
                        Some(e) => Some(e.0.upgrade().unwrap().clone()),
                        None => None
                    };
                }
            }
        }

        Element::<Key, ORDER>::set_connections(
            &new_element, prev_ptr.as_ref(), next_ptr.as_ref(), range
        );
        node.borrow_mut().size += 1;

        new_element
    }

    pub fn remove_element(&mut self, index: usize, range: f32) {
        let element = self.elements[index].as_mut().unwrap();
        element.borrow_mut().remove_connections(range);

        for i in index..(self.size - 1) {
            self.keys[i] = self.keys[i + 1].take();
            self.elements[i] = self.elements[i + 1].take();
        }

        self.size -= 1;
    }

    pub(crate) fn remove_element_without_shift(&mut self, index: usize, range: f32) {
        let element = self.elements[index].as_mut().unwrap();
        element.borrow_mut().remove_connections(range);
        self.elements[index] = None;
    }

    pub(crate) fn remove_element_soft(&mut self, index: usize) {
        for i in index..(self.size - 1) {
            self.keys[i] = self.keys[i + 1].take();
            self.elements[i] = self.elements[i + 1].take();
        }
        self.size -= 1;
    }

    pub(crate) fn find_child(&self, child: &Rc<RefCell<Self>>) -> Option<usize> {
        for i in 0..=self.size {
            if Rc::ptr_eq(self.children[i].as_ref().unwrap(), child) {
                return Some(i)
            }
        }
        None
    }

    pub(crate) fn shift_right(&mut self, index: usize) {
        for i in ((index + 1)..=self.size).rev() {
            self.keys[i] = self.keys[i - 1].take();
            self.elements[i] = self.elements[i - 1].take();
        }
    }

    pub(crate) fn shift_left_children(&mut self, index: usize) {
        for i in index..(Self::MAX_CHILDREN - 1) {
            self.children[i] = self.children[i + 1].take();
        }
    }

    pub(crate) fn shift_right_children(&mut self, index: usize) {
        for i in (index..=self.size).rev() {
            self.children[i + 1] = self.children[i].take();
        }
    }

    pub(crate) fn test_node(&self, print: bool) -> bool {
        let mut is_ok = true;
        for i in 0..self.size {
            if self.elements[i].is_none() { is_ok = false; }
            if self.keys[i].is_none() { is_ok = false; }
        }
        for i in 0..=self.size {
            if self.children[i].is_none() && !self.is_leaf { is_ok = false; }
        }
        
        if !is_ok && print {
            println!("wrong node size: {} is_leaf: {}", self.size, self.is_leaf);
            print!("   "); self.print_node_elements();
            print!("   "); self.print_node_children(false);
        }

        return is_ok
    }

    pub(crate) fn print_node(&self, borrow: bool) {
        println!("node size: {} is_leaf: {}", self.size, self.is_leaf);
        print!("   "); self.print_node_elements();
        print!("   "); self.print_node_children(borrow);
    }

    pub(crate) fn print_node_elements(&self) -> String {
        let mut node_string = format!("elements: ");
        for element in &self.elements {
            let s = match &element {
                Some(e) => format!(
                    "[{}:{}]", e.borrow().key, e.borrow().counter
                ),
                None => "[none]".to_string()
            };
            node_string.push_str(&s);
        }
        println!("{node_string}");
        node_string
    }

    pub(crate) fn print_node_children(&self, borrow: bool) -> String {
        let mut node_string = format!("children: ");
        for child in &self.children {
            let s = match &child {
                Some(c) => if borrow {
                    format!("[{}]", c.borrow().size)
                } else { "[some]".to_string() },
                None => "[none]".to_string()
            };
            node_string.push_str(&s);
        }
        println!("{node_string}");
        node_string
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
where Key: SensorData, [(); ORDER + 1]: {
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
                Some(e) => format!("{}", e.as_ref().borrow()),
                None => "[]".to_string()
            };
            node.push_str(&element);
        }
        node.push_str(", children:");
        for child in &self.children {
            let child_node: String = match child.as_ref() {
                Some(n) => format!("{}", n.as_ref().borrow()),
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
    use std::{
        rc::Rc,
        cell::RefCell
    };

    use witchnet_common::connection::collective::defining::{
        ConstantOneWeight
    };

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
        let graph = Rc::new(RefCell::new(ASAGraph::<i32, 3>::new(1)));
        let root: &Rc<RefCell<Node<i32, 3>>> = &graph.borrow().root;
        let graph_id = graph.borrow().id;

        root.borrow_mut().elements[0] = Some(Element::new(&2, 1, graph_id));
        root.borrow_mut().keys[0] = Some(2);
        root.borrow_mut().size = 1;

        let weighting_strategy = Rc::new(ConstantOneWeight);
        Node::insert_key_leaf(&root, &-1, 2, graph_id, 1f32, weighting_strategy.clone(), 0.00001, 1);
        Node::insert_key_leaf(&root, &1, 3, graph_id, 1f32, weighting_strategy.clone(), 0.00001, 1);
        root.borrow().insert_existing_key(&1, true);
        root.borrow().insert_existing_key(&-1, true);
        root.borrow().insert_existing_key(&2, true);
        root.borrow().insert_existing_key(&1, false);
        root.borrow().insert_existing_key(&-1, false);
        root.borrow().insert_existing_key(&2, false);
        root.borrow().insert_existing_key(&3, false);
       
        assert!(root.borrow().size == 3);
        assert_eq!(root.borrow().keys[0].unwrap(), -1);
        assert_eq!(root.borrow().keys[1].unwrap(), 1);
        assert_eq!(root.borrow().keys[2].unwrap(), 2);
        assert_eq!(root.borrow().elements[0].as_ref().unwrap().borrow().counter, 3);
        assert_eq!(root.borrow().elements[1].as_ref().unwrap().borrow().counter, 3);
        assert_eq!(root.borrow().elements[2].as_ref().unwrap().borrow().counter, 3);
    }

    #[test]
    fn split_node() {
        let graph = Rc::new(RefCell::new(ASAGraph::<i32, 3>::new(1)));
        let root: &Rc<RefCell<Node<i32, 3>>> = &graph.borrow().root;
        let graph_id = graph.borrow().id;

        root.borrow_mut().elements[0] = Some(Element::new(&1, 1, graph_id));
        root.borrow_mut().keys[0] = Some(1);
        root.borrow_mut().size = 1;

        let weighting_strategy = Rc::new(ConstantOneWeight);
        Node::insert_key_leaf(&root, &6, 2, graph_id, 5f32, weighting_strategy.clone(), 0.00001, 1);
        Node::insert_key_leaf(&root, &7, 3, graph_id, 6f32, weighting_strategy.clone(), 0.00001, 1);

        let root_new = Rc::new(RefCell::new(Node::new(false, None)));
        root_new.borrow_mut().children[0] = Some(root.clone());
        root.borrow_mut().parent = Some(Rc::downgrade(&root_new));
        Node::split_child(&root_new, 0);

        assert_eq!(root_new.borrow().size, 1);
        assert_eq!(root_new.borrow().children[0].as_ref().unwrap().borrow().size, 1);
        assert_eq!(root_new.borrow().children[1].as_ref().unwrap().borrow().size, 1);
        assert!(root_new.borrow().children[2].is_none());
        assert!(root_new.borrow().children[0].as_ref().unwrap().borrow().children[0].is_none());
        assert!(root_new.borrow().children[1].as_ref().unwrap().borrow().children[0].is_none());

        assert_eq!(root_new.borrow().keys[0].unwrap(), 6);
        assert_eq!(root_new.borrow().children[0].as_ref().unwrap().borrow().keys[0].unwrap(), 1);
        assert_eq!(root_new.borrow().children[1].as_ref().unwrap().borrow().keys[0].unwrap(), 7);
        assert!(root_new.borrow().keys[1].is_none());
        assert!(root_new.borrow().children[0].as_ref().unwrap().borrow().keys[1].is_none());
        assert!(root_new.borrow().children[1].as_ref().unwrap().borrow().keys[1].is_none());
        
        assert_eq!(root_new.borrow().elements[0].as_ref().unwrap().borrow().key, 6);
        assert_eq!(root_new.borrow().children[0].as_ref().unwrap().borrow().elements[0].as_ref().unwrap().borrow().key, 1);
        assert_eq!(root_new.borrow().children[1].as_ref().unwrap().borrow().elements[0].as_ref().unwrap().borrow().key, 7);
        assert!(root_new.borrow().elements[1].is_none());
        assert!(root_new.borrow().children[0].as_ref().unwrap().borrow().elements[1].is_none());
        assert!(root_new.borrow().children[1].as_ref().unwrap().borrow().elements[1].is_none());

        Node::insert_key_leaf(&root_new, &2, 4, graph_id, 6f32, weighting_strategy.clone(), 0.00001, 1);
        Node::insert_key_leaf(&root_new, &4, 5, graph_id, 6f32, weighting_strategy.clone(), 0.00001, 1);

        let middle_left_node = Rc::new(
            RefCell::new(Node::new(true, Some(Rc::downgrade(&root_new))))
        );
        middle_left_node.borrow_mut().elements[0] = Some(Element::new(&3, 6, graph_id));
        middle_left_node.borrow_mut().keys[0] = Some(3);
        middle_left_node.borrow_mut().size = 1;

        let middle_right_node = Rc::new(
            RefCell::new(Node::new(true, Some(Rc::downgrade(&root_new))))
        );
        middle_right_node.borrow_mut().elements[0] = Some(Element::new(&5, 7, graph_id));
        middle_right_node.borrow_mut().keys[0] = Some(5);
        middle_right_node.borrow_mut().size = 1;

        root_new.borrow_mut().children.swap(1, 3);
        root_new.borrow_mut().children[1] = Some(middle_left_node);
        root_new.borrow_mut().children[2] = Some(middle_right_node);

        let root_newer = Rc::new(RefCell::new(Node::new(false, None)));
        root_newer.borrow_mut().children[0] = Some(root_new.clone());
        root_new.borrow_mut().parent = Some(Rc::downgrade(&root_newer));
        Node::split_child(&root_newer, 0);

        let left_child = root_newer.borrow().children[0].as_ref().unwrap().clone();
        let right_child = root_newer.borrow().children[1].as_ref().unwrap().clone();
        
        let left_left_child = left_child.borrow().children[0].as_ref().unwrap().clone();
        let left_right_child = left_child.borrow().children[1].as_ref().unwrap().clone();
        let right_left_child = right_child.borrow().children[0].as_ref().unwrap().clone();
        let right_right_child = right_child.borrow().children[1].as_ref().unwrap().clone();

        assert_eq!(root_newer.borrow().size, 1);
        assert_eq!(left_child.borrow().size, 1);
        assert_eq!(right_child.borrow().size, 1);
        assert_eq!(left_left_child.borrow().size, 1);
        assert_eq!(left_right_child.borrow().size, 1);
        assert_eq!(right_left_child.borrow().size, 1);
        assert_eq!(right_right_child.borrow().size, 1);
        assert!(root_newer.borrow().children[2].is_none());
        assert!(left_child.borrow().children[2].is_none());
        assert!(right_child.borrow().children[2].is_none());
        assert!(left_left_child.borrow().children[0].is_none());
        assert!(left_right_child.borrow().children[0].is_none());
        assert!(right_left_child.borrow().children[0].is_none());
        assert!(right_right_child.borrow().children[0].is_none());
        assert_eq!(root_newer.borrow().is_leaf, false);
        assert_eq!(left_child.borrow().is_leaf, false);
        assert_eq!(right_child.borrow().is_leaf, false);
        assert_eq!(left_left_child.borrow().is_leaf, true);
        assert_eq!(left_right_child.borrow().is_leaf, true);
        assert_eq!(right_left_child.borrow().is_leaf, true);
        assert_eq!(right_right_child.borrow().is_leaf, true);
        
        assert!(root_newer.borrow().keys[1].is_none());
        assert!(left_child.borrow().keys[1].is_none());
        assert!(right_child.borrow().keys[1].is_none());
        assert!(left_left_child.borrow().keys[1].is_none());
        assert!(left_right_child.borrow().keys[1].is_none());
        assert!(right_left_child.borrow().keys[1].is_none());
        assert!(right_right_child.borrow().keys[1].is_none());
        
        assert!(root_newer.borrow().elements[1].is_none());
        assert!(left_child.borrow().elements[1].is_none());
        assert!(right_child.borrow().elements[1].is_none());
        assert!(left_left_child.borrow().elements[1].is_none());
        assert!(left_right_child.borrow().elements[1].is_none());
        assert!(right_left_child.borrow().elements[1].is_none());
        assert!(right_right_child.borrow().elements[1].is_none());

        assert_eq!(root_newer.borrow().keys[0].unwrap(), 4);
        assert_eq!(left_child.borrow().keys[0].unwrap(), 2);
        assert_eq!(right_child.borrow().keys[0].unwrap(), 6);
        assert_eq!(left_left_child.borrow().keys[0].unwrap(), 1);
        assert_eq!(left_right_child.borrow().keys[0].unwrap(), 3);
        assert_eq!(right_left_child.borrow().keys[0].unwrap(), 5);
        assert_eq!(right_right_child.borrow().keys[0].unwrap(), 7);

        assert_eq!(root_newer.borrow().elements[0].as_ref().unwrap().borrow().key, 4);
        assert_eq!(left_child.borrow().elements[0].as_ref().unwrap().borrow().key, 2);
        assert_eq!(right_child.borrow().elements[0].as_ref().unwrap().borrow().key, 6);
        assert_eq!(left_left_child.borrow().elements[0].as_ref().unwrap().borrow().key, 1);
        assert_eq!(left_right_child.borrow().elements[0].as_ref().unwrap().borrow().key, 3);
        assert_eq!(right_left_child.borrow().elements[0].as_ref().unwrap().borrow().key, 5);
        assert_eq!(right_right_child.borrow().elements[0].as_ref().unwrap().borrow().key, 7);
    }
}