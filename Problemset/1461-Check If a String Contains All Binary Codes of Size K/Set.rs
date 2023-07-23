use std::collections::HashSet;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }

        let mut x = i32::from_str_radix(&s[..k as usize], 2).unwrap();
        let y = 1 << k;
        let mut set = vec![x].into_iter().collect::<HashSet<_>>();

        for c in s.bytes().skip(k as usize) {
            x = (x << 1) % y + (c - b'0') as i32;
            set.insert(x);
        }

        set.len() == y as usize
    }
}
