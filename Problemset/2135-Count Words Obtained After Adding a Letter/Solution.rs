use std::collections::HashMap;

impl Solution {
    pub fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {
        let mut target_count = HashMap::new();
        let mut ret = 0;

        for target in target_words {
            let mut count = [0; 26];

            for ch in target.bytes() {
                count[(ch - b'a') as usize] += 1;
            }

            *target_count.entry(count).or_insert(0) += 1;
        }

        for start in start_words {
            let mut count = [0; 26];

            for ch in start.bytes() {
                count[(ch - b'a') as usize] += 1;
            }

            for i in 0..26 {
                if count[i] == 0 {
                    let mut tmp = count;
                    tmp[i] = 1;
                    ret += target_count.remove(&tmp).unwrap_or(0);
                }
            }
        }

        ret
    }
}
