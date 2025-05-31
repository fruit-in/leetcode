impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let mut lis_last = vec![0];
        let mut lis_left = vec![0; nums.len()];
        let mut lis_right = vec![0; nums.len()];

        for i in 0..nums.len() {
            let j = match lis_last.binary_search(&nums[i]) {
                Ok(j) => j,
                Err(j) if j == lis_last.len() => {
                    lis_last.push(0);
                    j
                }
                Err(j) => j,
            };

            lis_last[j] = nums[i];
            lis_left[i] = j;
        }

        lis_last = vec![0];
        for i in (0..nums.len()).rev() {
            let j = match lis_last.binary_search(&nums[i]) {
                Ok(j) => j,
                Err(j) if j == lis_last.len() => {
                    lis_last.push(0);
                    j
                }
                Err(j) => j,
            };

            lis_last[j] = nums[i];
            lis_right[i] = j;
        }

        (0..nums.len())
            .filter(|&i| lis_left[i] > 1 && lis_right[i] > 1)
            .map(|i| nums.len() + 1 - lis_left[i] - lis_right[i])
            .min()
            .unwrap() as i32
    }
}
