mod easy;
mod medium;

fn main() {
    use std::{
        rc::Rc,
        cell::RefCell
    };
    
    use easy::two_sum;
    use medium::{
        validate_bst, 
        group_anagrams,
        merge_intervals
    };
    

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


    // GROUP ANAGRAMS:
    let v: Vec<String> = vec!["eat","tea","tan","ate","nat","bat"]
        .into_iter()
        .map(|i| i.into())
        .collect();
    println!("raw: {:?}\ngrouped: {:?}", v, group_anagrams::Solution::group_anagrams(v.clone()));  // for simplicity just .clone(), ik it can be optimized


    // MERGE INTERVALS:
    let intervals_arr: Vec<[i32; 2]> = vec![[1,3], [2,6] ,[8,10] ,[8,9], [9,11],[15,18], [2,4] ,[16,17]];
    let intervals: Vec<Vec<i32>> = intervals_arr
        .iter()
        .map(|arr| arr.to_vec())
        .collect(); 
    let r: Vec<Vec<i32>> = merge_intervals::Solution::merge(intervals);
    println!("raw: {:?}\nmerged: {:?}", intervals_arr, r);
}


#[test]
fn testing() {
    let intervals_arr: Vec<[i32; 2]> = vec![[1,3], [2,6] ,[8,10] ,[8,9], [9,11],[15,18], [2,4] ,[16,17]];
    let intervals: Vec<Vec<i32>> = intervals_arr
        .iter()
        .map(|arr| arr.to_vec())
        .collect();
    let r: Vec<Vec<i32>> = medium::merge_intervals::Solution::merge(intervals);
    println!("raw: {:?}\nmerged: {:?}", intervals_arr, r);
}