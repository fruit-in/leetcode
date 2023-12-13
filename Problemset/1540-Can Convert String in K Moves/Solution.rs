impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count = [0; 26];

        for (ch0, ch1) in s.bytes().zip(t.bytes()) {
            count[(ch1 + 26 - ch0) as usize % 26] += 1;
        }

        (1..26).all(|x| 26 * (count[x] - 1) + x as i32 <= k)
    }
}
