use std::collections::HashSet;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut b2 = HashSet::new();
        let mut a = 0_i32;

        while let Some(a2) = a.checked_mul(a) {
            if a2 > c {
                break;
            }

            b2.insert(a2);

            if b2.contains(&(c - a2)) {
                return true;
            }

            a += 1;
        }

        false
    }
}
