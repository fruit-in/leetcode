impl Solution {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut prefix = vec![0; n];
        let mut suffix = vec![0; n];
        let mut count = (1, 1);

        for i in 1..n {
            prefix[i] = count.0;
            suffix[n - 1 - i] = count.1;

            if nums[i] <= nums[i - 1] {
                count.0 += 1;
            } else {
                count.0 = 1;
            }
            if nums[n - 1 - i] <= nums[n - i] {
                count.1 += 1;
            } else {
                count.1 = 1;
            }
        }

        (k..n - k)
            .filter(|&i| prefix[i].min(suffix[i]) >= k)
            .map(|i| i as i32)
            .collect()
    }
}
