impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let mut diagonal_nums = (0..nums.len())
            .map(|i| [nums[i][i], nums[i][nums.len() - i - 1]])
            .flatten()
            .collect::<Vec<_>>();
        diagonal_nums.sort_unstable();

        *diagonal_nums
            .iter()
            .rev()
            .find(|&&num| {
                for x in 2..=(num as f64).sqrt() as i32 {
                    if num % x == 0 {
                        return false;
                    }
                }

                num > 1
            })
            .unwrap_or(&0)
    }
}
