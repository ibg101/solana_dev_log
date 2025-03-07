pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());

        intervals.sort_unstable_by_key(|interval| interval[0]);

        for interval in intervals {
            if let Some(prev_interval) = results.last_mut() {
                let prev_max: &mut i32 = &mut prev_interval[1];
                let current_min: i32 = interval[0];
                let current_max: i32 = interval[1];

                // overlap check, example: [1, 3], [2, 4], where 3 > 2 => they overlap
                if *prev_max >= current_min {
                    *prev_max = (*prev_max).max(current_max);
                    continue;
                }
            }
            results.push(interval); 
        }        

        results.shrink_to_fit();
        results
    }
}