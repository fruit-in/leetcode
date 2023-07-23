impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let mut x = -k - 1;
        let mut ret = 0;
        nums.sort_unstable();

        for num in nums {
            if num - x > k {
                x = num;
                ret += 1;
            }
        }

        ret
    }
}
