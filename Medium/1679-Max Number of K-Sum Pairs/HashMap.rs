use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for num in nums {
            if *count.get(&(k - num)).unwrap_or(&0) > 0 {
                *count.get_mut(&(k - num)).unwrap() -= 1;
                ret += 1;
            } else {
                *count.entry(num).or_insert(0) += 1;
            }
        }

        ret
    }
}
