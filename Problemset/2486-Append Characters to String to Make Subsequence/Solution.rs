impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = 0;

        for j in 0..s.len() {
            if i >= t.len() {
                break;
            } else if t[i] == s[j] {
                i += 1;
            }
        }

        (t.len() - i) as i32
    }
}
