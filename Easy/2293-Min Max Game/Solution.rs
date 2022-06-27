impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut n = nums.len();

        while n > 1 {
            n /= 2;
            nums = (0..n)
                .map(|i| match i % 2 {
                    0 => nums[2 * i].min(nums[2 * i + 1]),
                    _ => nums[2 * i].max(nums[2 * i + 1]),
                })
                .collect();
        }

        nums[0]
    }
}
