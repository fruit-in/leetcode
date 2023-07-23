use std::collections::HashMap;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut counter = vec![(0, 1)].into_iter().collect::<HashMap<_, _>>();
        let mut count = 0;
        let mut ret = 0;

        for i in 0..nums.len() {
            if nums[i] % 2 == 1 {
                count += 1;
            }
            ret += counter.get(&(count - k)).unwrap_or(&0);
            *counter.entry(count).or_insert(0) += 1;
        }

        ret
    }
}
