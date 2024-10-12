use std::collections::HashMap;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        if !nums.contains(&k) {
            return 0;
        }

        let i = nums.iter().position(|&x| x == k).unwrap();
        let mut diff = 0;
        let mut count = HashMap::from([(0, 1)]);
        let mut ret = 1;

        for j in i + 1..nums.len() {
            if nums[j] > k {
                diff += 1;
            } else {
                diff -= 1;
            }
            if diff == 0 || diff == 1 {
                ret += 1;
            }
            *count.entry(diff).or_insert(0) += 1;
        }

        diff = 0;

        for j in (0..i).rev() {
            if nums[j] > k {
                diff += 1;
            } else {
                diff -= 1;
            }
            ret += count.get(&-diff).unwrap_or(&0);
            ret += count.get(&(1 - diff)).unwrap_or(&0);
        }

        ret
    }
}
