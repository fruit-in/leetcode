use std::collections::HashMap;

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut count = HashMap::new();

        for &num in &nums {
            let (a, b, c) = *count.get(&(num - 1)).unwrap_or(&(0, 0, 0));
            let (d, e, f) = *count.get(&num).unwrap_or(&(0, 0, 0));

            if a > 0 {
                count.insert(num - 1, (a - 1, b, c));
                count.insert(num, (d, e + 1, f));
            } else if b > 0 {
                count.insert(num - 1, (a, b - 1, c));
                count.insert(num, (d, e, f + 1));
            } else if c > 0 {
                count.insert(num - 1, (a, b, c - 1));
                count.insert(num, (d, e, f + 1));
            } else {
                count.insert(num, (d + 1, e, f));
            }
        }

        count.values().all(|&(a, b, _)| a | b == 0)
    }
}
