impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut a = 0;
        let mut b = c.min(46340);

        while a <= b {
            if c - b * b > a * a {
                a += 1;
            } else if c - b * b < a * a {
                b -= 1;
            } else {
                return true;
            }
        }

        false
    }
}
