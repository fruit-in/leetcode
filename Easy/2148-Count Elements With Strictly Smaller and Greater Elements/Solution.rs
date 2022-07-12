impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();

        nums.iter().filter(|&num| num != min && num != max).count() as i32
    }
}
