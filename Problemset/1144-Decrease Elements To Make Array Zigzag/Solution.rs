impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        Self::moves_to_make_v(&nums, true).min(Self::moves_to_make_v(&nums, false))
    }

    fn moves_to_make_v(nums: &[i32], odd: bool) -> i32 {
        (odd as usize..nums.len())
            .step_by(2)
            .map(|i| {
                nums[i]
                    - nums[i]
                        .min(*nums.get(i - 1).unwrap_or(&1000) - 1)
                        .min(*nums.get(i + 1).unwrap_or(&1000) - 1)
            })
            .sum()
    }
}
