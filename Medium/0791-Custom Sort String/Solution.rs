impl Solution {
    pub fn custom_sort_string(s: String, t: String) -> String {
        let mut indices = [26; 26];
        let mut t = t.into_bytes();

        for (i, ch) in s.bytes().enumerate() {
            indices[(ch - b'a') as usize] = i;
        }
        t.sort_unstable_by_key(|k| indices[(k - b'a') as usize]);

        String::from_utf8(t).unwrap()
    }
}
