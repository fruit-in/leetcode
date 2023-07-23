impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut x = num;
        let mut ret = 0;

        while x > 0 {
            if num % (x % 10) == 0 {
                ret += 1;
            }
            x /= 10;
        }

        ret
    }
}
