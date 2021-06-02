impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut diff = std::i32::MAX;
        nums.sort_unstable();

        for i in 0..nums.len() - 2 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];

                if (target - sum).abs() < diff.abs() {
                    diff = target - sum;
                }
                if sum < target {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }

        target - diff
    }
}
