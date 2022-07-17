impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc | x)
    }
}
