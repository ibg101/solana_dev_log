use std::collections::HashMap;


// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]

// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

        for (index_x, x) in nums.into_iter().enumerate() {
            let y: i32 = target - x;
            if let Some(index_y) = hash_map.get(&y) {
                return Vec::from_iter([index_x as i32, *index_y as i32]);
            }
            hash_map.insert(x, index_x);
        }

        // this solution always guarantees, that there will be a correct pair (if the args are set correctly)
        panic!("Incorrect arguments! Check the example in two_sum.rs!");
    }
}