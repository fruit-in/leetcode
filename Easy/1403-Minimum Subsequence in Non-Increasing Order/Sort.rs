impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let half = nums.iter().sum::<i32>() / 2;
        let mut acc = 0;
        nums.sort_unstable_by(|a, b| b.cmp(a));

        nums.into_iter()
            .take_while(|x| {
                acc += x;
                acc - x <= half
            })
            .collect()
    }
}
