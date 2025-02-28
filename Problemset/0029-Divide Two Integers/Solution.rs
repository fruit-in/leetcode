impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let is_neg = (dividend < 0 && divisor > 0) || (dividend > 0 && divisor < 0);
        let mut dividend = (dividend as i64).abs();
        let divisor = (divisor as i64).abs();
        let mut ret = 0;

        while dividend >= divisor {
            for i in 1..=32 {
                if divisor << i > dividend {
                    dividend -= divisor << (i - 1);
                    ret += 1 << (i - 1);
                    break;
                }
            }
        }

        if is_neg {
            ret = -ret;
        }

        ret
    }
}
