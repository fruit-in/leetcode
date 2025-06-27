use std::collections::HashMap;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut count = HashMap::new();
        let mut indices_sum = HashMap::new();
        let mut arr = vec![0; nums.len()];

        for i in 0..nums.len() {
            *count.entry(nums[i]).or_insert(0) += 1;
            *indices_sum.entry(nums[i]).or_insert(0) += i as i64;
            arr[i] = count[&nums[i]] * i as i64 - indices_sum[&nums[i]];
        }

        count.clear();
        indices_sum.clear();

        for i in (0..nums.len()).rev() {
            *count.entry(nums[i]).or_insert(0) += 1;
            *indices_sum.entry(nums[i]).or_insert(0) += i as i64;
            arr[i] += indices_sum[&nums[i]] - count[&nums[i]] * i as i64;
        }

        arr
    }
}
