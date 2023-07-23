impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let p = p.as_bytes();
        let mut count = 1;
        let mut max_len = [0; 26];

        for i in 0..p.len() {
            if i == 0 || (p[i] + 26 - p[i - 1]) % 26 != 1 {
                count = 1;
            } else {
                count += 1;
            }
            for j in 0..26.min(count) {
                let k = (p[i + 1 - count + j] - b'a') as usize;
                max_len[k] = max_len[k].max(count - j);
            }
        }

        max_len.iter().sum::<usize>() as i32
    }
}
