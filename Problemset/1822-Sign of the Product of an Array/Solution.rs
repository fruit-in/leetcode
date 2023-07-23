impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut sign = 1;

        for x in nums {
            if x == 0 {
                return 0;
            } else if x < 0 {
                sign *= -1;
            }
        }

        sign
    }
}
