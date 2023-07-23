impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut n = n;
        let mut zeroes = 0;
        while n >= 5 {
            n /= 5;
            zeroes += n;
        }
        zeroes
    }
}
