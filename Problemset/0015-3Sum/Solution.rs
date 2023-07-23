use std::collections::HashSet;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut triplets = HashSet::new();
        nums.sort_unstable();

        for i in 0..nums.len() - 2 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                if nums[i] + nums[j] + nums[k] == 0 {
                    triplets.insert(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                } else if nums[i] + nums[j] + nums[k] < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }

        triplets.into_iter().collect()
    }
}
