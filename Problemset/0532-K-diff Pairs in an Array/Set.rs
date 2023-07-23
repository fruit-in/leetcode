use std::collections::HashSet;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        if k < 0 {
            return 0;
        }

        let mut nums_set = HashSet::new();
        let mut pairs_j = HashSet::new();

        for n in nums {
            if nums_set.contains(&(n - k)) {
                pairs_j.insert(n);
            }
            if nums_set.contains(&(n + k)) {
                pairs_j.insert(n + k);
            }

            nums_set.insert(n);
        }

        pairs_j.len() as i32
    }
}
