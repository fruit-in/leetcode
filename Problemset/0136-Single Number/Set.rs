use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for n in nums {
            if set.contains(&n) {
                set.remove(&n);
            } else {
                set.insert(n);
            }
        }
        *set.iter().next().unwrap()
    }
}
