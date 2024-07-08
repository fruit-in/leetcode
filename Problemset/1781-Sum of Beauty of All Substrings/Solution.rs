impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ret = 0;

        for i in 0..s.len() {
            let mut count = [0; 26];

            for j in i..s.len() {
                count[(s[j] - b'a') as usize] += 1;

                let max_freq = count.iter().max().unwrap();
                let min_freq = count.iter().filter(|&&x| x > 0).min().unwrap();

                ret += max_freq - min_freq;
            }
        }

        ret
    }
}
