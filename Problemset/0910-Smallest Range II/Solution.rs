impl Solution {
    pub fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut nums = nums.into_iter().map(|x| x + k).collect::<Vec<_>>();
        let mut ret = 0;

        nums.sort_unstable();

        if nums[n - 1] - 2 * k < nums[0] {
            ret = nums[n - 1] - nums[0];
        } else {
            for i in 1..nums.len() {
                if nums[i] - 2 * k >= nums[0] {
                    ret = nums[i - 1].max(nums[n - 1] - 2 * k) - nums[0];
                    break;
                }
            }
        }

        for i in 1..nums.len() {
            if nums[i] - 2 * k > nums[0] {
                break;
            }

            ret = ret.min(nums[i - 1].max(nums[n - 1] - 2 * k) - nums[i] + 2 * k);
        }

        ret
    }
}
