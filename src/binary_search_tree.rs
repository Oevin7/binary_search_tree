
pub struct Node<T: Ord> {
    data : T,
    left_node : Option<Box<Node<T>>>,
    right_node : Option<Box<Node<T>>>
}

impl<T: Ord> Node<T> {
    pub fn new(data : T) -> Option<Box<Node<T>>> {
        let node = Box::new(Node {
            data,
            left_node : None,
            right_node : None,
        });

       Some(node)

    }

    pub fn get_value(&self) -> &T {
        &self.data
    }

    pub fn get_left_node(&self) -> &Option<Box<Node<T>>> {
        &self.left_node
    }

    pub fn get_left_node_mut(&mut self) -> &mut Option<Box<Node<T>>> {
        &mut self.left_node
    }

    pub fn get_right_node(&self) -> &Option<Box<Node<T>>> {
        &self.right_node
    }

    pub fn get_right_node_mut(&mut self) -> &mut Option<Box<Node<T>>> {
        &mut self.right_node
    }

    pub fn empty_children(&self) -> bool {
        if self.left_node.is_none() && self.right_node.is_none() {
            return true
        }
        false
    }

    pub fn set_left_node(&mut self, node : Option<Box<Node<T>>>) {
        self.left_node = node;
    }

    pub fn set_right_node(&mut self, node : Option<Box<Node<T>>>) {
        self.left_node = node;
    }
}

pub struct BinarySearchTree<T: Ord> {
    root : Option<Box<Node<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            root : None,
        }
    }

    pub fn new_with_root(root : Option<Box<Node<T>>>) -> BinarySearchTree<T> {
        BinarySearchTree {
            root
        }
    }

    pub fn root_value(&self) -> Option<&T> {
        match &self.root {
            Some(node) => Some(node.get_value()),
            None => None
        }
    }

    pub fn get_root_node(&self) -> &Option<Box<Node<T>>> {
        &self.root
    }

    fn get_root_node_mut(&mut self) -> &mut Option<Box<Node<T>>> {
        &mut self.root
    }

    fn set_root_node(&mut self, node : Option<Box<Node<T>>>) {
        self.root = node;
    }

    pub fn is_empty(&self) -> bool {
        if self.root.is_none() {
            return true
        }
        false
    }

    pub fn insert(&mut self, node : Option<Box<Node<T>>>) {
        if self.root.is_none() {
            self.set_root_node(node);
        } else {
            self.recursive_insert(node)
        }
    }

    fn recursive_insert(&mut self, node: Option<Box<Node<T>>>) {
        let root = match self.get_root_node_mut() {
            Some(val) => val,
            None => {
                eprintln!("Could not get the root value. Aborting.");
                return;
            }
        };
        let node_val = match node {
            Some(val) => val.get_value(),
            None => {
                eprintln!("Aborting. Could not get value.");
                return;
            }
        };

        let root_value = root.get_value();
        let mut left = None;

        if node_val < root_value {
            left = root.get_left_node();
            match left {
                Some(node) => {
                    self.recursive_insert(node)
                }
                None => root.set_left_node(*node)
            }
        }

    }
}