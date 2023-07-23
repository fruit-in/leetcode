impl Solution {
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();
        let mut ranks = Vec::new();

        for score in nums {
            match sorted_nums.len() - sorted_nums.binary_search(&score).unwrap() - 1 {
                0 => ranks.push("Gold Medal".to_string()),
                1 => ranks.push("Silver Medal".to_string()),
                2 => ranks.push("Bronze Medal".to_string()),
                i => ranks.push((i + 1).to_string()),
            };
        }

        ranks
    }
}
