impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut cnt = [0; 26];
        let mut ret = Vec::new();

        for ch in s.bytes() {
            cnt[(ch - b'a') as usize] += 1;
        }

        while ret.len() < s.len() {
            for i in 0..26 {
                if cnt[i] > 0 {
                    ret.push(i as u8 + b'a');
                    cnt[i] -= 1;
                }
            }
            for i in (0..26).rev() {
                if cnt[i] > 0 {
                    ret.push(i as u8 + b'a');
                    cnt[i] -= 1;
                }
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
