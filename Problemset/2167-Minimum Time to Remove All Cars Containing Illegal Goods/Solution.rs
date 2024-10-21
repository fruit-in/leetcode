impl Solution {
    pub fn minimum_time(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = 0;
        let mut x = 1;
        let mut indices_r = vec![];
        let mut ret = i32::MAX;

        for i in (0..s.len()).rev() {
            if s[i] == b'1' {
                indices_r.push(i);
            }
        }

        if indices_r.is_empty() {
            return 0;
        }

        for i in 0..s.len() {
            if s[i] == b'1' {
                indices_r.pop();
            }
            count += (s[i] - b'0') as i32;
            x = x.min(i as i32 - 2 * count + 2);
            ret = ret
                .min(2 * count - 1 + x + (s.len() - indices_r.last().unwrap_or(&s.len())) as i32);
        }

        ret
    }
}
