mod binary_search_tree;

use crate::binary_search_tree::*;

fn main() {
    let node = Node::new(7);

    let bst_with_root = BinarySearchTree::new_with_root(node);

    let root_val = bst_with_root.root_value().unwrap();
    println!("{root_val}");
}
