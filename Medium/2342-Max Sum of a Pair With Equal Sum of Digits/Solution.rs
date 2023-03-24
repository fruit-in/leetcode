use std::collections::HashMap;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut max_pairs = HashMap::new();

        for &num in &nums {
            let digits_sum = num
                .to_string()
                .bytes()
                .map(|d| (d - b'0') as i32)
                .sum::<i32>();
            let pair = max_pairs.entry(digits_sum).or_insert((0, 0));

            if num >= pair.1 {
                *pair = (pair.1, num);
            } else if num > pair.0 {
                *pair = (num, pair.1);
            }
        }

        max_pairs
            .values()
            .filter(|&&(x, y)| x > 0)
            .map(|(x, y)| x + y)
            .max()
            .unwrap_or(-1)
    }
}
