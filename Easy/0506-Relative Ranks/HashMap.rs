use std::collections::HashMap;

impl Solution {
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable_by(|a, b| b.cmp(a));
        let mut score_ranks = HashMap::new();

        for i in 0..sorted_nums.len() {
            match i {
                0 => score_ranks.insert(sorted_nums[i], "Gold Medal".to_string()),
                1 => score_ranks.insert(sorted_nums[i], "Silver Medal".to_string()),
                2 => score_ranks.insert(sorted_nums[i], "Bronze Medal".to_string()),
                _ => score_ranks.insert(sorted_nums[i], (i + 1).to_string()),
            };
        }

        nums.iter().map(|x| score_ranks.get(x).unwrap().to_string()).collect()
    }
}
