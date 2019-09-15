impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut z = 0;
        for n in nums {
            z ^= n;
        }
        z
    }
}
