impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;

        for j in 0..nums.len() {
            if i < 2 || nums[j] != nums[i - 2] {
                nums[i] = nums[j];
                i += 1;
            }
        }

        i as i32
    }
}
