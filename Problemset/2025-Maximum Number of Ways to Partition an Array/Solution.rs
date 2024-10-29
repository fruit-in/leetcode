use std::collections::HashMap;

impl Solution {
    pub fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64;
        let mut sum = nums[0] as i64;
        let mut prefix_sum = nums[0] as i64;
        let mut suffix_sum = 0;
        let mut prefix_count = HashMap::from([(prefix_sum, 1)]);
        let mut suffix_count = HashMap::new();
        let mut ret = 0;

        for i in (1..n).rev() {
            sum += nums[i] as i64;
            suffix_sum += nums[i] as i64;
            *suffix_count.entry(suffix_sum).or_insert(0) += 1;
        }

        if sum % 2 == 0 {
            ret = *suffix_count.get(&(sum / 2)).unwrap_or(&0);
        }

        if (suffix_sum + k) % 2 == 0 {
            ret = ret.max(*suffix_count.get(&((suffix_sum + k) / 2)).unwrap_or(&0));
        }

        for i in 1..n {
            let new_sum = sum - nums[i] as i64 + k;

            *suffix_count.get_mut(&suffix_sum).unwrap() -= 1;
            suffix_sum -= nums[i] as i64;

            if new_sum % 2 == 0 {
                let x = *prefix_count.get(&(new_sum / 2)).unwrap_or(&0);
                let y = *suffix_count.get(&(new_sum / 2)).unwrap_or(&0);
                ret = ret.max(x + y);
            }

            prefix_sum += nums[i] as i64;
            *prefix_count.entry(prefix_sum).or_insert(0) += 1;
        }

        ret
    }
}
