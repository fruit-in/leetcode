use std::collections::HashSet;

impl Solution {
    pub fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        let mut set = HashSet::new();
        let mut ret = 1;

        for x in &rolls {
            set.insert(x);
            if set.len() as i32 == k {
                set = HashSet::new();
                ret += 1;
            }
        }

        ret
    }
}
