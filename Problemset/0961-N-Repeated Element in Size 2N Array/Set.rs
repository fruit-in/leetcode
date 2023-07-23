use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut set = HashSet::new();

        for n in a {
            if set.contains(&n) {
                return n;
            } else {
                set.insert(n);
            }
        }

        -1
    }
}
