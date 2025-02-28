impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let mut stack = vec![];
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for c in p.bytes() {
            if c != b'*' {
                stack.push(c);
            } else {
                match stack.pop() {
                    Some(b'.') => stack.push(b','),
                    Some(c) if c >= b'a' => stack.push(c - b'a' + b'A'),
                    _ => (),
                }
            }
        }

        for &c in &stack {
            let mut tmp = vec![false; s.len() + 1];

            if c == b',' || c.is_ascii_uppercase() {
                tmp[0] = dp[0];
            }

            for i in 1..=s.len() {
                match c {
                    b'.' => tmp[i] = dp[i - 1],
                    b'a'..=b'z' => tmp[i] = c == s[i - 1] && dp[i - 1],
                    _ => {
                        tmp[i] = dp[i];
                        for j in (0..i).rev() {
                            if !tmp[i] && (c == b',' || c - b'A' + b'a' == s[j]) {
                                tmp[i] = dp[j];
                            } else {
                                break;
                            }
                        }
                    }
                }
            }

            dp = tmp;
        }

        dp[s.len()]
    }
}
