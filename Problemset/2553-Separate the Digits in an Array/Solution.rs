impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];

        for num in nums {
            for digit in num.to_string().bytes() {
                ret.push((digit - b'0') as i32);
            }
        }

        ret
    }
}
