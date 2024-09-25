use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut count = HashMap::new();

        for s in &strs {
            let s = s.as_bytes();
            let mut subs = HashSet::new();

            for x in 1..2_i32.pow(s.len() as u32) {
                let mut sub = vec![];

                for i in 0..s.len() {
                    if x & (1 << i) != 0 {
                        sub.push(s[i]);
                    }
                }

                subs.insert(sub);
            }

            for sub in subs.into_iter() {
                *count.entry(sub).or_insert(0) += 1;
            }
        }

        count
            .iter()
            .filter(|&(_, c)| *c == 1)
            .map(|(sub, _)| sub.len() as i32)
            .max()
            .unwrap_or(-1)
    }
}
