impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().filter(|&&x| x % 6 == 0).sum::<i32>();
        let len = nums.iter().filter(|&&x| x % 6 == 0).count() as i32;

        sum.checked_div(len).unwrap_or(0)
    }
}
