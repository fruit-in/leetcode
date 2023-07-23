impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut count = 0;
        let mut ret = 0;

        for i in 0..nums.len() {
            count += 1;
            if nums[i] != 0 {
                count = 0;
            }
            ret += count;
        }

        ret
    }
}
