impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums = nums.into_iter().enumerate().collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        nums.truncate(k as usize);
        nums.sort_unstable();

        nums.into_iter().map(|(_, num)| num).collect()
    }
}
