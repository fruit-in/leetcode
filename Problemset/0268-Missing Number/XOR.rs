impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut miss = 0;
        for i in 0..nums.len() {
            miss ^= nums[i] ^ (i as i32 + 1);
        }
        miss
    }
}
