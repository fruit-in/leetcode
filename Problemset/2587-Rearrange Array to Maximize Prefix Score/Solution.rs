impl Solution {
    pub fn max_score(mut nums: Vec<i32>) -> i32 {
        let mut prefix = vec![0; nums.len() + 1];

        nums.sort_unstable_by_key(|x| -x);

        for i in 0..nums.len() {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        prefix.iter().filter(|&&x| x > 0).count() as i32
    }
}
