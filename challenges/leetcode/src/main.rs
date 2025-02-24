mod easy;

fn main() {
    let nums: Vec<i32> = Vec::from_iter([4, 3, 2]);
    assert_eq!(easy::two_sum::Solution::two_sum(nums, 6), vec![2, 0]);
}
