impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut max = nums[0];
        let mut ret = -1;

        for &num in &nums[1..] {
            if num < min {
                min = num;
                max = num;
            } else if num > max {
                max = num;
                ret = ret.max(max - min);
            }
        }

        ret
    }
}
