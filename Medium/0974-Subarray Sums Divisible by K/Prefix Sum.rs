use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut counter = vec![(0, 1)].into_iter().collect::<HashMap<_, _>>();
        let mut sum = 0;
        let mut ret = 0;

        for x in nums {
            sum = ((sum + x) % k + k) % k;
            let count = counter.entry(sum).or_insert(0);
            ret += *count;
            *count += 1;
        }

        ret
    }
}
