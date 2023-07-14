impl Solution {
    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums;
        let mut k = k;
        let mut ret = 0;

        nums.push(0);
        nums.push(i32::MAX);
        nums.sort_unstable();

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                continue;
            } else if nums[i] - nums[i - 1] - 1 < k {
                k -= nums[i] - nums[i - 1] - 1;
                ret +=
                    (nums[i - 1] as i64 + nums[i] as i64) * (nums[i] - nums[i - 1] - 1) as i64 / 2;
            } else {
                ret += (nums[i - 1] as i64 * 2 + 1 + k as i64) * k as i64 / 2;
                break;
            }
        }

        ret
    }
}
