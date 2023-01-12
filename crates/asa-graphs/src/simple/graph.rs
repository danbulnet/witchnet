use std::{
    fmt::Display,
    rc::Rc,
    cell::RefCell,
    io::{ self, Write }
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

    fn update_min_max(&mut self, key: &Key) -> bool {
        let (key_min, key_max) = match self.extreme_keys(){
            Some((key_min, key_max)) => (
                *dyn_clone::clone_box(key_min), 
                *dyn_clone::clone_box(key_max)
            ),
            None => return false
        };
        if key == &key_min {
            let next_element_min = match &self.element_min.as_ref().unwrap().borrow().next {
                Some(el) => Some(el.upgrade().unwrap().clone()),
                None => None
            };
            self.element_min = next_element_min;
            self.key_min = match &self.element_min {
                Some(el) => Some(*dyn_clone::clone_box(&el.borrow().key)),
                None => None
            };
        }
        if key == &key_max {
            let next_element_max = match &self.element_max.as_ref().unwrap().borrow().prev {
                Some(el) => Some(el.upgrade().unwrap().clone()),
                None => None
            };
            self.element_max = next_element_max;
            self.key_max = match &self.element_max {
                Some(el) => Some(*dyn_clone::clone_box(&el.borrow().key)),
                None => None
            };
        }
        true
    }

    pub fn remove(&mut self, key: &Key) -> bool {
        if !self.update_min_max(key) { return false }

        // 1. Use the search operation to find an element containing the key intended for removal.
        let mut current_node = Some(self.root.clone());
        if current_node.as_ref().unwrap().borrow().size == 0 { return false }

        let mut parent: Option<Rc<RefCell<Node<Key, ORDER>>>> = None;
        let mut parent_of_parent: Option<Rc<RefCell<Node<Key, ORDER>>>> = None;
        let mut parent_index: Option<usize> = None;
        let mut child_index: Option<usize> = None;

        while let Some(node) = &current_node.clone() {
            let mut index = 0;
            {
                let node = node.borrow();
                while index < node.size && key > node.keys[index].as_ref().unwrap() {
                    index += 1;
                }
            }

            if index < node.borrow().size && key == node.borrow().keys[index].as_ref().unwrap() {
                let element = node.borrow().elements[index].as_ref().unwrap().clone();
                if element.borrow().counter == 1 {
                    // 3. I f the element storing the removed key is a leaf
                    if node.borrow().is_leaf {
                        // 3. remove the element storing this key from this leaf, switch pointers
                        //    from its predecessor and successor to point themselves 
                        //    as direct neighbors.
                        node.borrow_mut().remove_element(index);
                         // 3. Next, if this leaf is not empty, finish the remove operation, 
                         //    else go to step 6 to fill or remove this empty leaf
                        if node.borrow().size > 0 {
                            return true
                        } else {
                            // 6. Empty leaf
                            if !Rc::ptr_eq(&node, &self.root) {
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
                        let mut left_leaf = node.borrow().children[index].clone();
                        let mut right_leaf = node.borrow().children[index + 1].clone();
                        let mut left_leaf_parent = Some(node.clone());
                        let mut right_leaf_parent = Some(node.clone());
                        let mut left_leaf_parent_of_parent = parent.clone();
                        let mut right_leaf_parent_of_parent = parent.clone();
                        let mut left_parent_size = node.borrow().size;
                        let mut right_parent_size = node.borrow().size;

                        while !left_leaf.as_ref().unwrap().borrow().is_leaf {
                            left_parent_size = left_leaf.as_ref().unwrap().borrow().size;
                            right_parent_size = right_leaf.as_ref().unwrap().borrow().size;
                            left_leaf_parent_of_parent = left_leaf_parent.take();
                            right_leaf_parent_of_parent = right_leaf_parent.take();
                            left_leaf_parent = left_leaf.clone();
                            right_leaf_parent = right_leaf.clone();
                            left_leaf = {
                                let left_leaf_deref = left_leaf.as_ref().unwrap().borrow();
                                left_leaf_deref.children[left_leaf_deref.size].clone()
                            };
                            right_leaf = {
                                let right_leaf_deref = right_leaf.as_ref().unwrap().borrow();
                                right_leaf_deref.children[0].clone()
                            };
                        }
                        // 4. If one of these leaves contains more than one element, 
                        //    replace the removed element in the non leaf node by this
                        //    connected neighbor element from the leaf containing 
                        //    more than one element, and finish the remove operation
                        let left_leaf_size = left_leaf.as_ref().unwrap().borrow().size;
                        let right_leaf_size = right_leaf.as_ref().unwrap().borrow().size;
                        if left_leaf_size >= 2 {
                            let mut node_deref = node.borrow_mut();
                            let mut left_leaf = left_leaf.as_mut().unwrap().borrow_mut();
                            node_deref.remove_element_without_shift(index);
                            node_deref.keys[index] = left_leaf.keys[left_leaf_size - 1].take();
                            node_deref.elements[index] = left_leaf.elements[left_leaf_size - 1].take();
                            left_leaf.size -= 1;
                            return true
                        } else if right_leaf_size >= 2 {
                            let mut node_deref = node.borrow_mut();
                            let mut right_leaf = right_leaf.as_mut().unwrap().borrow_mut();
                            node_deref.remove_element_without_shift(index);
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
                                    let mut left_leaf = left_leaf.as_mut().unwrap().borrow_mut();
                                    let mut node_deref = node.borrow_mut();
                                    node_deref.elements[index].as_mut().unwrap().borrow_mut()
                                        .remove_connections();
                                    node_deref.keys[index] = left_leaf.keys[0].take();
                                    node_deref.elements[index] = left_leaf.elements[0].take();
                                    left_leaf.size -= 1;
                                }
                                // 6. Empty leaf
                                let left_leaf_parent_ptr = left_leaf_parent.as_ref().unwrap();
                                let leaf_index = if Rc::ptr_eq(left_leaf_parent_ptr, node) { 
                                    index 
                                } else {
                                    match left_leaf_parent_ptr.try_borrow() {
                                        Ok(b) => b.size,
                                        Err(_) => node.borrow().size,
                                    }
                                };
                                let leaf_parent_index = if left_leaf_parent_of_parent.is_some() &&
                                    Rc::ptr_eq(
                                        left_leaf_parent_of_parent.as_ref().unwrap(), &node
                                    ) { Some(index) } else {
                                    if left_leaf_parent_of_parent.is_some() && 
                                        parent.is_some() && Rc::ptr_eq(
                                            left_leaf_parent_of_parent.as_ref().unwrap(), 
                                            &parent.as_ref().unwrap()
                                    ) { child_index } else {
                                        if left_leaf_parent_of_parent.is_some() {
                                            Some(
                                                left_leaf_parent_of_parent.as_ref()
                                                    .unwrap().borrow().size
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
                                    let mut right_leaf = right_leaf.as_mut().unwrap().borrow_mut();
                                    let mut node_deref = node.borrow_mut();
                                    node_deref.elements[index].as_mut().unwrap().borrow_mut()
                                        .remove_connections();
                                    node_deref.keys[index] = right_leaf.keys[0].take();
                                    node_deref.elements[index] = right_leaf.elements[0].take();
                                    right_leaf.size -= 1;
                                }
                                // 6. Empty leaf
                                let leaf_index = if Rc::ptr_eq(
                                    left_leaf_parent.as_ref().unwrap(), &node
                                ) { index + 1 } else { 0 };
                                let leaf_parent_index = if left_leaf_parent_of_parent.is_some() &&
                                    Rc::ptr_eq(
                                        left_leaf_parent_of_parent.as_ref().unwrap(), &node
                                ) { Some(index + 1) } else {
                                    if left_leaf_parent_of_parent.is_some() && 
                                        parent.is_some() && Rc::ptr_eq(
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
                    element.borrow_mut().counter -= 1;
                    return true
                }
            } else if node.borrow().is_leaf {
                //  1. If this key is not found, finish this operation with no effect
                return false
            } else {
                parent_of_parent = parent;
                parent = Some(node.clone());
                current_node = node.borrow().children[index].clone();
                parent_index = child_index;
                child_index = Some(index);
            }
        }
        false
    }

    fn empty_node(
        &mut self, 
        node: Rc<RefCell<Node<Key, ORDER>>>,
        index: usize,
        parent: Option<Rc<RefCell<Node<Key, ORDER>>>>,
        parent_index: Option<usize>
    ) {
        // 6. If one of the nearest siblings of the empty leaf contains 
        //    more than one element, replace its ancestor element from
        //    the empty leaf side by the closest element from this sibling
        //    and shift this ancestor element to the empty node
        //    Next, finish the remove operation
        let empty_node = node.borrow().children[index].as_ref().unwrap().clone();
        let sibling_node = node.borrow().children[1].clone();
        let second_sibling_node = if index == 0 {
            None
        } else { node.borrow().children[index - 1].clone() };
        
        let mut node_deref = node.borrow_mut();
        let mut empty_node_deref = empty_node.borrow_mut();
        let node_size = node_deref.size;
        
        if index == 0 && sibling_node.as_ref().unwrap().borrow().size > 1 {
            let mut sibling_node_deref = sibling_node.as_ref().unwrap().borrow_mut();
            
            empty_node_deref.keys[0] = node_deref.keys[0].take();
            empty_node_deref.elements[0] = node_deref.elements[0].take();
            empty_node_deref.size += 1;

            node_deref.keys[0] = sibling_node_deref.keys[0].take();
            node_deref.elements[0] = sibling_node_deref.elements[0].take();
            sibling_node_deref.remove_element_soft(0);

            return
        } else if index == node_size 
            && second_sibling_node.as_ref().unwrap().borrow().size > 1 {
            let mut sibling_node_deref = second_sibling_node.as_ref().unwrap().borrow_mut();
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
            if l_sibling.is_some() && l_sibling.as_ref().unwrap().borrow().size > 1 {
                let mut l_sibling_deref = l_sibling.as_ref().unwrap().borrow_mut();
                let l_sibling_size = l_sibling_deref.size;

                empty_node_deref.keys[0] = node_deref.keys[index - 1].take();
                empty_node_deref.elements[0] = node_deref.elements[index - 1].take();
                empty_node_deref.size += 1;

                node_deref.keys[index - 1] = l_sibling_deref.keys[l_sibling_size - 1].take();
                node_deref.elements[index - 1] 
                    = l_sibling_deref.elements[l_sibling_size - 1].take();
                l_sibling_deref.remove_element_soft(l_sibling_size - 1);

                return
            } else if r_sibling.is_some() && r_sibling.as_ref().unwrap().borrow().size > 1 {
                let mut r_sibling_deref = r_sibling.as_ref().unwrap().borrow_mut();

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
                        let mut r_sibling_deref = r_sibling.as_ref().unwrap().borrow_mut();
                        
                        r_sibling_deref.shift_right(0);
                        r_sibling_deref.keys[0] = node_deref.keys[index].take();
                        r_sibling_deref.elements[0] = node_deref.elements[index].take();
                        r_sibling_deref.size += 1;
                        node_deref.remove_element_soft(index);
                        node_deref.children[index] = None;
                        node_deref.shift_left_children(0);
                        
                        return
                    } else {
                        let mut l_sibling_deref = l_sibling.as_ref().unwrap().borrow_mut();
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
                        let mut r_sibling_deref = r_sibling.as_ref().unwrap().borrow_mut();
                        node_deref.shift_right(1);
                        node_deref.keys[1] = r_sibling_deref.keys[0].take();
                        node_deref.elements[1] = r_sibling_deref.elements[0].take();
                        node_deref.size += 1;
                        node_deref.is_leaf = true;
                        node_deref.children[0] = None;
                        node_deref.children[1] = None;
                    } else {
                        let mut l_sibling_deref = l_sibling.as_ref().unwrap().borrow_mut();
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
                    if !Rc::ptr_eq(&node, &self.root) {
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
        node: Rc<RefCell<Node<Key, ORDER>>>, 
        mut node_deref: &mut Node<Key, ORDER>, 
        parent: Rc<RefCell<Node<Key, ORDER>>>,
        parent_index: usize
    ) {
        // 9. If one of the sibling node of the reduced subtree root contains 
        //    more than one element, shift it closest element to the parent 
        //    replacing the closest element in it, create a new child
        //    and move down the replaced element from t he parent to this new child. 
        //    Connect this new child to the reduced subtree and switch the closest child
        //    of the sibling to this new child as well. Next, finish
        // let mut node_deref = node.borrow_mut();
        let mut parent_deref = parent.borrow_mut();

        let l_sibling = if parent_index != 0 {
            parent_deref.children[parent_index - 1].clone()
        } else { None };
        let r_sibling = if parent_index + 1 <= parent_deref.size {
            parent_deref.children[parent_index + 1].clone()
        } else { None };
        
        if l_sibling.is_some() && l_sibling.as_ref().unwrap().borrow().size > 1 {
            let mut l_sibling_deref = l_sibling.as_ref().unwrap().borrow_mut();
            let l_sibling_size = l_sibling_deref.size;

            let new_node = Rc::new(RefCell::new(Node::<Key, ORDER>::new(false, None)));
            let mut new_node_deref = new_node.borrow_mut();
            new_node_deref.keys[0] = parent_deref.keys[parent_index - 1].take();
            new_node_deref.elements[0] = parent_deref.elements[parent_index - 1].take();
            new_node_deref.size += 1;
            new_node_deref.children[0] = l_sibling_deref.children[l_sibling_size].take();
            new_node_deref.children[1] = Some(node.clone());

            new_node_deref.parent = Some(Rc::downgrade(&parent));
            node_deref.parent = Some(Rc::downgrade(&new_node));
            new_node_deref.children[0].as_mut().unwrap().borrow_mut().parent 
                = Some(Rc::downgrade(&new_node));

            parent_deref.children[parent_index] = Some(new_node.clone());
            parent_deref.keys[parent_index - 1] 
                = l_sibling_deref.keys[l_sibling_size - 1].take();
            parent_deref.elements[parent_index - 1] 
                = l_sibling_deref.elements[l_sibling_size - 1].take();
            l_sibling_deref.size -= 1;

            return
        } else if r_sibling.is_some() && r_sibling.as_ref().unwrap().borrow().size > 1 {
            let mut r_sibling_deref = r_sibling.as_ref().unwrap().borrow_mut();

            let new_node = Rc::new(RefCell::new(Node::<Key, ORDER>::new(false, None)));
            let mut new_node_deref = new_node.borrow_mut();
            new_node_deref.keys[0] = parent_deref.keys[parent_index].take();
            new_node_deref.elements[0] = parent_deref.elements[parent_index].take();
            new_node_deref.size += 1;
            new_node_deref.children[0] = Some(node.clone());
            new_node_deref.children[1] = r_sibling_deref.children[0].take();

            new_node_deref.parent = Some(Rc::downgrade(&parent));
            node_deref.parent = Some(Rc::downgrade(&new_node));
            new_node_deref.children[1].as_mut().unwrap().borrow_mut().parent 
                = Some(Rc::downgrade(&new_node));

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
            let mut l_sibling_deref = l_sibling.borrow_mut();
            let mut l_sibling_size = l_sibling_deref.size;

            l_sibling_deref.keys[l_sibling_size] = parent_deref.keys[parent_index - 1].take();
            l_sibling_deref.elements[l_sibling_size] 
                = parent_deref.elements[parent_index - 1].take();
            l_sibling_deref.size += 1;
            l_sibling_size += 1;
            l_sibling_deref.children[l_sibling_size] = Some(node.clone());
            parent_deref.remove_element_soft(parent_index - 1);
            parent_deref.shift_left_children(parent_index);
            node_deref.parent = Some(Rc::downgrade(&l_sibling));
        } else if let Some(r_sibling) = r_sibling {
            let mut r_sibling_deref = r_sibling.borrow_mut();

            r_sibling_deref.shift_right(0);
            r_sibling_deref.keys[0] = parent_deref.keys[parent_index].take();
            r_sibling_deref.elements[0] = parent_deref.elements[parent_index].take();
            r_sibling_deref.size += 1;
            r_sibling_deref.shift_right_children(0);
            r_sibling_deref.children[0] = Some(node.clone());
            parent_deref.remove_element_soft(parent_index);
            parent_deref.shift_left_children(parent_index);
            node_deref.parent = Some(Rc::downgrade(&r_sibling));
        }
        if parent_deref.size >= 1 {
            return
        } else if Rc::ptr_eq(&parent, &self.root) {
            self.root = parent_deref.children[0].as_ref().unwrap().clone();
            self.root.borrow_mut().parent = None;
            return
        } else {
            let parent_of_parent = parent_deref.parent.clone();
            let reduced_subree_root = parent_deref.children[0].as_ref().unwrap().clone();
            let mut reduced_subree_root_deref = reduced_subree_root.borrow_mut();

            if let Some(parent_of_parent_weak) = &parent_of_parent {
                let parent_of_parent = parent_of_parent_weak.upgrade().unwrap();

                let parent_of_parent_index = parent_of_parent.borrow().find_child(&parent);
                parent_of_parent.borrow_mut().children[parent_of_parent_index.unwrap()] 
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

    pub fn test_graph(&self, print_if_ok: bool) -> bool {
        let mut height = 0;
        let mut node = self.root.clone();
        let mut queue: Vec<Vec<Rc<RefCell<Node<Key, ORDER>>>>> = vec![vec![]];
        queue[0].push(node.clone());

        let mut is_ok = true;

        loop {
            queue.push(vec![]);
            for i in 0..(queue[height].len()) {
                node = queue[height][i].clone();
                let node_size = node.borrow().size;
                for j in 0..(node_size) {
                    if !node.borrow().is_leaf {
                        queue[height + 1].push(node.borrow().children[j].as_ref().unwrap().clone());
                    }
                }
                if !node.borrow().is_leaf {
                    let node_deref = node.borrow();
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
                        if !node.borrow().test_node(true) { is_ok = false; }
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
    use std::time::Instant;
    
    use rand::{
        Rng, 
        seq::SliceRandom,
        rngs::StdRng,
        SeedableRng
    };
    
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

    #[test]
    fn levels() {
        let mut graph = ASAGraph::<i32, 3>::new("test");
        for i in 1..=25 { graph.insert(&i); }
        let lvlvs: Vec<Vec<Vec<i32>>> = graph.levels().into_iter().map(
            |v| v.into_iter().map(
                |n| n.into_iter().map(
                    |e| {
                        e.borrow().key
                    }
                ).collect()
            ).collect()
        ).collect();
        println!("{:?}", lvlvs);
    }

    #[test]
    fn remove() {
        let mut rng = StdRng::seed_from_u64(42);

        let mut graph = ASAGraph::<i32, 5>::new("test");

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