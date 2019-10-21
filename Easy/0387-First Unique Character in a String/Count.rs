impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut cnt = [0; 26];

        for ch in s.bytes() {
            cnt[(ch - b'a') as usize] += 1;
        }

        for (i, ch) in s.bytes().enumerate() {
            if cnt[(ch - b'a') as usize] == 1 {
                return i as i32;
            }
        }

        -1
    }
}
