impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return String::new();
        }

        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut count_s = [0; 52];
        let mut count_t = [0; 52];
        let mut sub_indices = (0, s.len() + 1);
        let mut i = 0;
        let mut j = 0;

        while j < t.len() {
            match s[j] {
                b'A'..=b'Z' => count_s[(s[j] - b'A') as usize] += 1,
                _ => count_s[(s[j] - b'a') as usize + 26] += 1,
            }
            match t[j] {
                b'A'..=b'Z' => count_t[(t[j] - b'A') as usize] += 1,
                _ => count_t[(t[j] - b'a') as usize + 26] += 1,
            }

            j += 1;
        }

        if (0..52).all(|k| count_s[k] >= count_t[k]) {
            sub_indices = (i, j);
        }

        while j < s.len() {
            while j < s.len() && (0..52).any(|k| count_s[k] < count_t[k]) {
                match s[j] {
                    b'A'..=b'Z' => count_s[(s[j] - b'A') as usize] += 1,
                    _ => count_s[(s[j] - b'a') as usize + 26] += 1,
                }

                j += 1;
            }

            while (0..52).all(|k| count_s[k] >= count_t[k]) {
                if j - i < sub_indices.1 - sub_indices.0 {
                    sub_indices = (i, j);
                }

                match s[i] {
                    b'A'..=b'Z' => count_s[(s[i] - b'A') as usize] -= 1,
                    _ => count_s[(s[i] - b'a') as usize + 26] -= 1,
                }

                i += 1;
            }
        }

        if sub_indices.1 - sub_indices.0 > s.len() {
            return String::new();
        }

        String::from_utf8(s[sub_indices.0..sub_indices.1].to_vec()).unwrap()
    }
}
