impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let mut count = 0;
        let mut ret = s.len();

        for (i, c) in s.chars().enumerate() {
            count += ((i % 2 == 0) ^ (c == '0')) as usize;
        }

        for c in s.chars() {
            if c == '0' {
                count = s.len() - count - s.len() % 2;
            } else {
                count = s.len() - count + s.len() % 2;
            }

            ret = ret.min(count).min(s.len() - count);
        }

        ret as i32
    }
}
