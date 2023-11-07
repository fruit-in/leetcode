impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut k = k as i64 - 1;
        let mut chars = vec![];
        let mut length = 0;

        for ch in s.bytes() {
            chars.push((ch, length));

            if ch.is_ascii_lowercase() {
                length += 1;
            } else {
                length *= (ch - b'0') as i64;
            }

            if length > k {
                break;
            }
        }

        while let Some((ch, i)) = chars.pop() {
            if ch.is_ascii_lowercase() {
                if i == k {
                    return String::from_utf8(vec![ch]).unwrap();
                }

                length -= 1;
            } else {
                length /= (ch - b'0') as i64;
                k %= length;
            }
        }

        unreachable!()
    }
}
