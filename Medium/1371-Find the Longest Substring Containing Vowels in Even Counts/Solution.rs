use std::collections::HashMap;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut even = [true; 5];
        let mut first_appear = vec![(even, -1)].into_iter().collect::<HashMap<_, _>>();
        let mut ret = 0;

        for i in 0..s.len() {
            match s[i] {
                b'a' => even[0] = !even[0],
                b'e' => even[1] = !even[1],
                b'i' => even[2] = !even[2],
                b'o' => even[3] = !even[3],
                b'u' => even[4] = !even[4],
                _ => (),
            }

            match first_appear.get(&even) {
                Some(j) => ret = ret.max(i as i32 - j),
                None => {
                    first_appear.insert(even, i as i32);
                }
            }
        }

        ret
    }
}
