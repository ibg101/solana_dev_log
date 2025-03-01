use std::{
    rc::Rc,
    cell::{Ref, RefCell}   
};


#[derive(Default, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

pub struct Solution;

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, ..Default::default() }
    }
}

impl Solution {
    // condition: root.val > left.val && root.val < right.val
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::validate(&root, i64::MIN, i64::MAX)
    }

    fn validate(unchecked_node: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(node) = unchecked_node {
            let node_ref: Ref<TreeNode> = node.borrow();
            let node_val: i64 = node_ref.val as i64;
            
            return node_val > min && node_val < max
                && Self::validate(&node_ref.left, min, node_val)
                && Self::validate(&node_ref.right, node_val, max)
        }

        true  // empty tree is a valid BST
    }
}