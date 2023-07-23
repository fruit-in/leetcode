impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut dup = 0;
        let mut miss = 0;

        for i in 0..nums.len() {
            let j = nums[i].abs() as usize - 1;
            if nums[j] < 0 {
                dup = nums[i].abs();
            } else {
                nums[j] = -nums[j];
            }
        }

        for i in 0..nums.len() {
            if nums[i] > 0 {
                miss = i as i32 + 1;
                break;
            }
        }

        vec![dup, miss]
    }
}
