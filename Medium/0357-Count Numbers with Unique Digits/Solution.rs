impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let factorials = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
        let mut n = n.min(10) as usize;
        let mut ret = 1;

        while n > 0 {
            ret += 9 * factorials[9] / factorials[10 - n];
            n -= 1;
        }

        ret
    }
}
