impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut cnt = [0; 26];

        for ch in s.bytes() {
            cnt[(ch - b'a') as usize] += 1;
        }

        for ch in t.bytes() {
            cnt[(ch - b'a') as usize] -= 1;
            if cnt[(ch - b'a') as usize] == -1 {
                return ch as char;
            }
        }

        ' '
    }
}
