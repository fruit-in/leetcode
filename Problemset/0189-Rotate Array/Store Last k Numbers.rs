impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        let last_k = nums[(nums.len() - k)..].to_vec();
        for i in (k..nums.len()).rev() {
            nums.swap(i, i - k);
        }
        nums.splice(..k, last_k);
    }
}
