impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut s = s.into_bytes();
        let n = s.len();
        let mut i = 0;

        if k > 1 {
            s.sort_unstable();
        } else {
            for j in 1..n {
                for k in 0..n {
                    if s[(i + k) % n] < s[(j + k) % n] {
                        break;
                    } else if s[(i + k) % n] > s[(j + k) % n] {
                        i = j;
                        break;
                    }
                }
            }

            s.rotate_left(i);
        }

        String::from_utf8(s).unwrap()
    }
}
