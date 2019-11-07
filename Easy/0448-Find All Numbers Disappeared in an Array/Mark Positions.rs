impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ret = Vec::new();

        for i in 0..nums.len() {
            let j = nums[i].abs() as usize - 1;
            nums[j] = -nums[j].abs();
        }

        for i in 0..nums.len() {
            if nums[i] > 0 {
                ret.push(i as i32 + 1);
            }
        }

        ret
    }
}
