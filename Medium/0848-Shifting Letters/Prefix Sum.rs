impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut s = s.into_bytes();
        let mut shift = 0;

        for i in (0..s.len()).rev() {
            shift = (shift + shifts[i]) % 26;
            s[i] = (s[i] - b'a' + (shift as u8)) % 26 + b'a';
        }

        String::from_utf8(s).unwrap()
    }
}
