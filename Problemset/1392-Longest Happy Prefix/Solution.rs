impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let s = s.as_bytes();
        let mut j = 0;
        let mut next = vec![0; s.len()];

        for i in 1..s.len() {
            while j > 0 && s[i] != s[j] {
                j = next[j - 1];
            }

            if s[i] == s[j] {
                j += 1;
                next[i] = j;
            }
        }

        String::from_utf8(s[..next[s.len() - 1]].to_vec()).unwrap()
    }
}
