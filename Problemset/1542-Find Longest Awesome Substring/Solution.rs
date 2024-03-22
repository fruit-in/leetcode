use std::collections::HashMap;

impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut hash = HashMap::from([(0, -1)]);
        let mut x = 0;
        let mut ret = 1;

        for (i, digit) in s.bytes().enumerate() {
            x ^= 1 << (digit - b'0');
            ret = ret.max(i as i32 - hash.get(&x).unwrap_or(&(i as i32)));
            for digit in 0..10 {
                ret = ret.max(i as i32 - hash.get(&(x ^ (1 << digit))).unwrap_or(&(i as i32)));
            }
            if !hash.contains_key(&x) {
                hash.insert(x, i as i32);
            }
        }

        ret
    }
}
