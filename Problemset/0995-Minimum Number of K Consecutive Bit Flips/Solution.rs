impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut flip = vec![false; nums.len()];
        let mut window_xor = false;
        let mut ret = 0;

        for i in 0..nums.len() {
            if (nums[i] == 0 && !window_xor) || (nums[i] == 1 && window_xor) {
                if i + k - 1 < nums.len() {
                    flip[i] = true;
                    ret += 1;
                } else {
                    return -1;
                }
            }
            window_xor ^= flip[i];
            if i >= k - 1 {
                window_xor ^= flip[i - k + 1];
            }
        }

        ret
    }
}
