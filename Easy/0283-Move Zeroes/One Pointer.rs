impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for _ in 0..nums.len() {
            if nums[i] == 0 {
                nums.remove(i);
                nums.push(0);
            } else {
                i += 1;
            }
        }
    }
}
