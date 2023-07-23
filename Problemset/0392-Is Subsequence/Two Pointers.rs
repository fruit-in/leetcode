impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s = s.chars();
        let mut t = t.chars();
        while let Some(ch_s) = s.next() {
            loop {
                match t.next() {
                    Some(ch_t) => {
                        if ch_t == ch_s {
                            break;
                        }
                    },
                    None => return false,
                };
            }
        }
        true
    }
}
