use std::collections::HashMap;

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        let s = s.as_bytes();
        let min_size = min_size as usize;
        let mut letter_count = [0; 26];
        let mut unique_count = 0;
        let mut substring_count = HashMap::new();

        for i in 0..min_size {
            letter_count[(s[i] - b'a') as usize] += 1;
            unique_count += (letter_count[(s[i] - b'a') as usize] == 1) as i32;
        }
        if unique_count <= max_letters {
            substring_count.insert(&s[..min_size], 1);
        }

        for i in min_size..s.len() {
            letter_count[(s[i - min_size] - b'a') as usize] -= 1;
            unique_count -= (letter_count[(s[i - min_size] - b'a') as usize] == 0) as i32;
            letter_count[(s[i] - b'a') as usize] += 1;
            unique_count += (letter_count[(s[i] - b'a') as usize] == 1) as i32;
            if unique_count <= max_letters {
                *substring_count.entry(&s[i - min_size + 1..=i]).or_insert(0) += 1;
            }
        }

        *substring_count.values().max().unwrap_or(&0)
    }
}
