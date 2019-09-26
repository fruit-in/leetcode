impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let sum: i32 = nums.iter().sum();
        n * (n + 1) / 2 - sum
    }
}
