use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
        let mut max_len = 0;

        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }

        for (num, cnt1) in counter.iter() {
            if let Some(&cnt2) = counter.get(&(num + 1)) {
                max_len = max_len.max(cnt1 + cnt2);
            }
        }

        max_len
    }
}
