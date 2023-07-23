impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;

        while i < j && s[i] == s[j] {
            let c = s[i];
            while i < j && s[i] == c {
                i += 1;
            }
            if i == j {
                return 0;
            }
            while i < j && s[j] == c {
                j -= 1;
            }
        }

        (j - i + 1) as i32
    }
}
