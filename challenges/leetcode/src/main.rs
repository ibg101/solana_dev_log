mod easy;
mod medium;

fn main() {
    use std::{
        rc::Rc,
        cell::RefCell
    };
    
    use easy::two_sum;
    use medium::validate_bst;
    

    // TWO_SUM:
    let nums: Vec<i32> = Vec::from_iter([4, 3, 2]);
    assert_eq!(two_sum::Solution::two_sum(nums, 6), vec![2, 0]);


    // VALIDATE BINARY SEARCH TREE:
    // Remember - condition: root.val > left.val && root.val < right.val
    let validate_bst_values: [i32; 3] = [2, 1, 3];
    assert_eq!(validate_bst_values.len(), 3);

    let root: validate_bst::TreeNode = validate_bst::TreeNode {
        val: validate_bst_values[0],
        left: Some(Rc::new(RefCell::new(validate_bst::TreeNode::new(validate_bst_values[1])))),
        right: Some(Rc::new(RefCell::new(validate_bst::TreeNode::new(validate_bst_values[2]))))
    };
    assert!(validate_bst::Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))));
}
