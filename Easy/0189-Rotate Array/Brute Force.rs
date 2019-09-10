impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            for i in (1..nums.len()).rev() {
                nums.swap(i, i - 1);
            }
        }
    }
}
