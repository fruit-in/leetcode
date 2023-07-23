impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ret = 0;

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            if nums[i + 1..].binary_search(&(nums[i] + k)).is_ok() {
                ret += 1;
            }
        }

        ret
    }
}
