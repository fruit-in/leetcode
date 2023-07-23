impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut s = s.into_bytes();

        for i in 0..s.len() {
            if s[i] == b'?' {
                if i == 0 {
                    if s.len() == 1 || s[1] != b'a' {
                        s[0] = b'a';
                    } else {
                        s[0] = b'b';
                    }
                } else {
                    s[i] = s[i - 1] % 26 + b'a';
                    if i + 1 < s.len() && s[i + 1] == s[i] {
                        s[i] = s[i] % 26 + b'a';
                    }
                }
            }
        }

        String::from_utf8(s).unwrap()
    }
}
