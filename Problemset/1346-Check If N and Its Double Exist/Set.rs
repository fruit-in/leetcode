use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        for n in arr {
            if set.contains(&(2 * n)) || (n % 2 == 0 && set.contains(&(n / 2))) {
                return true;
            }
            set.insert(n);
        }

        false
    }
}
