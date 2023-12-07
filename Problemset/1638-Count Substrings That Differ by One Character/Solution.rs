impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut ret = 0;

        for i in 0..s.len() {
            for j in 0..t.len() {
                let mut count = 0;

                for k in 0..(s.len() - i).min(t.len() - j) {
                    if s[i + k] != t[j + k] {
                        count += 1;
                    }
                    if count > 1 {
                        break;
                    } else if count == 1 {
                        ret += 1;
                    }
                }
            }
        }

        ret
    }
}
