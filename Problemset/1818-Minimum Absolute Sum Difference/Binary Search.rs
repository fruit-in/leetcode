impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut x = 0;
        let mut nums = nums1.iter().zip(nums2.iter()).collect::<Vec<_>>();
        nums.sort_unstable();

        for i in 0..nums.len() {
            let y = (nums[i].1 - nums[i].0).abs();
            let z = match nums.binary_search_by_key(&nums[i].1, |&(a, _)| a) {
                Ok(_) => 0,
                Err(0) => nums[0].0 - nums[i].1,
                Err(j) if j == nums.len() => nums[i].1 - nums[j - 1].0,
                Err(j) => (nums[i].1 - nums[j - 1].0).min(nums[j].0 - nums[i].1),
            };

            sum = (sum + y) % 1_000_000_007;
            x = x.max(y - z);
        }

        (sum - x).rem_euclid(1_000_000_007)
    }
}
