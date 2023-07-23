use std::collections::HashSet;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut set = HashSet::new();

        for i in 0..20 {
            for j in 0..20 {
                let pow_int = x.saturating_pow(i).saturating_add(y.saturating_pow(j));

                if pow_int <= bound {
                    set.insert(pow_int);
                }
            }
        }

        set.drain().collect()
    }
}
