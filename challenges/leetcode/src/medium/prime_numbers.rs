pub struct Solution {}

impl Solution {
    // Example 1:
    // Input: left = 10, right = 19
    // Output: [11,13]
    // Explanation: The prime numbers between 10 and 19 are 11, 13, 17, and 19.
    // The closest gap between any pair is 2, which can be achieved by [11,13] or [17,19].
    // Since 11 is smaller than 17, we return the first pair.

    // Example 2:
    // Input: left = 4, right = 6
    // Output: [-1,-1]
    // Explanation: There exists only one prime number in the given range, so the conditions cannot be satisfied.
    
    pub fn closest_in_range(left: i32, right: i32) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::new();

        for num in left..=right {
            if num < 2 || num != 2 && num % 2 == 0 {
                continue;
            }

            let mut is_prime: bool = true;

            // using square root of the number instead of the exact number
            for i in 2..=(num as f64).sqrt() as i32 {
                if num % i == 0 {
                    is_prime = false;
                    break;
                }
            }

            if is_prime {
                results.push(num);
            }
        }

        match results.len() {
            num if num == 2 => results,
            num if num > 2 => {
                let (mut min_diff, mut index): (i32, usize) = (i32::MAX, 0);  
                
                for (i, number) in results.iter().enumerate() {
                    if let Some(next_number) = results.get(i + 1) {
                        let current_diff: i32 = next_number - number;

                        if current_diff < min_diff {
                            min_diff = current_diff;
                            index = i;
                        }
                    }
                }
                
                vec![results[index], results[index + 1]]
            },
            _ => vec![-1, -1]
        }
    }
}