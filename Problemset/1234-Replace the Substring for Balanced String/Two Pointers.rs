impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = [0; 4];
        let mut l = 0;
        let mut ret = s.len();

        for c in s {
            match c {
                b'Q' => count[0] += 1,
                b'W' => count[1] += 1,
                b'E' => count[2] += 1,
                _ => count[3] += 1,
            }
        }

        for r in 0..=s.len() {
            while l <= r && 4 * count.iter().max().unwrap() - count.iter().sum::<usize>() <= r - l {
                ret = ret.min(r - l);
                match s[l.min(s.len() - 1)] {
                    b'Q' => count[0] += 1,
                    b'W' => count[1] += 1,
                    b'E' => count[2] += 1,
                    _ => count[3] += 1,
                }
                l += 1;
            }
            match s[r.min(s.len() - 1)] {
                b'Q' => count[0] -= 1,
                b'W' => count[1] -= 1,
                b'E' => count[2] -= 1,
                _ => count[3] -= 1,
            }
        }

        ret as i32
    }
}
