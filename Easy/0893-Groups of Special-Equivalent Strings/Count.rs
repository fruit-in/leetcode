use std::collections::HashSet;

impl Solution {
    pub fn num_special_equiv_groups(a: Vec<String>) -> i32 {
        let mut set = HashSet::new();

        for s in a {
            let mut cnt = vec![0; 52];
            for (i, c) in s.bytes().enumerate() {
                cnt[(c - b'a') as usize + 26 * (i % 2)] += 1;
            }
            set.insert(cnt);
        }

        set.len() as i32
    }
}
