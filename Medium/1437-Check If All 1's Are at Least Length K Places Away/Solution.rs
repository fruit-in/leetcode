impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut prev_1 = -k - 1;

        for i in 0..nums.len() {
            if nums[i] == 1 {
                if i as i32 - prev_1 <= k {
                    return false;
                }
                prev_1 = i as i32;
            }
        }

        true
    }
}
