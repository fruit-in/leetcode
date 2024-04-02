impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut count = [0; 26];
        let mut most_ch = b'A';
        let mut i = -1;
        let mut ret = 0;

        for j in 0..s.len() {
            count[(s[j] - b'A') as usize] += 1;
            if count[(s[j] - b'A') as usize] > count[(most_ch - b'A') as usize] {
                most_ch = s[j];
            }
            while j as i32 - i - count[(most_ch - b'A') as usize] > k {
                i += 1;
                count[(s[i as usize] - b'A') as usize] -= 1;
                if s[i as usize] == most_ch {
                    most_ch = (0..26).max_by_key(|&i| count[i]).unwrap() as u8 + b'A';
                }
            }

            ret = ret.max(j as i32 - i);
        }

        ret
    }
}
