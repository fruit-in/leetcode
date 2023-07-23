impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut answer = vec![0; queries.len()];

        nums.sort_unstable();
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }

        for i in 0..queries.len() {
            answer[i] = match nums.binary_search(&queries[i]) {
                Ok(j) => j as i32 + 1,
                Err(j) => j as i32,
            };
        }

        answer
    }
}
