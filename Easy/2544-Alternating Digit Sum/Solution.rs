impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let mut n = n;
        let mut ret = 0;

        while n > 0 {
            ret = n % 10 - ret;
            n /= 10;
        }

        ret
    }
}
