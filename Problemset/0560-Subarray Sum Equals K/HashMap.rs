use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = vec![(0, 1)].into_iter().collect::<HashMap<_, _>>();
        let mut sum = 0;
        let mut ret = 0;

        for num in nums {
            sum += num;
            ret += count.get(&(sum - k)).unwrap_or(&0);
            *count.entry(sum).or_insert(0) += 1;
        }

        ret
    }
}
