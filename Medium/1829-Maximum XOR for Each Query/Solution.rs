impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut xor = nums.iter().fold(0, |acc, x| acc ^ x);
        let mut answer = vec![(1 << maximum_bit) - 1; nums.len()];

        for i in 0..nums.len() {
            answer[i] ^= xor;
            xor ^= nums[nums.len() - 1 - i];
        }

        answer
    }
}
