impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut count = 0;
        let mut ret = 0;
        nums.sort_unstable();
        let mut prev = nums[0];

        for &num in &nums[1..] {
            if num != prev {
                count += 1;
                prev = num;
            }

            ret += count;
        }

        ret
    }
}
