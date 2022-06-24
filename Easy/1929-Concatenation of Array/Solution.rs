impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        nums.clone().into_iter().chain(nums.into_iter()).collect()
    }
}
