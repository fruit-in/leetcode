use std::collections::HashMap;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let mut n = nums.len();
        let mut counti = HashMap::new();
        let mut ret = 0;

        for k in 2..n - 1 {
            let mut count = 0;

            for j in 0..k {
                if nums[j] > nums[k] && count > 0 {
                    counti.insert((j, k), count);
                } else if nums[j] < nums[k] {
                    count += 1;
                }
            }
        }

        for j in 1..n - 2 {
            let mut count = 0;

            for k in (j + 1..n).rev() {
                if nums[k] < nums[j] && count > 0 {
                    ret += count * counti.get(&(j, k)).unwrap_or(&0);
                } else if nums[k] > nums[j] {
                    count += 1;
                }
            }
        }

        ret
    }
}
