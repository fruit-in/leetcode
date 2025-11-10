impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut asc = nums[(1..nums.len()).find(|&i| nums[i] != nums[0]).unwrap_or(0)] < nums[0];
        let mut ret = 1;

        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] && (asc ^ (nums[i] > nums[i - 1])) {
                asc = !asc;
                ret += 1;
            }
        }

        ret
    }
}
