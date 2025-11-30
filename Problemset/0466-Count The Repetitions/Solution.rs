use std::collections::HashMap;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut i = 0;
        let mut memo = HashMap::new();
        let mut count2 = 0;

        for _ in 0..n1 {
            if let Some(&(count, j)) = memo.get(&i) {
                count2 += count;
                i = j;
                continue;
            }

            let init = i;
            let mut count = 0;

            for j in 0..s1.len() {
                if s1[j] == s2[i] {
                    i = (i + 1) % s2.len();
                    if i == 0 {
                        count += 1;
                    }
                }
            }

            memo.insert(init, (count, i));
            count2 += count;
        }

        count2 / n2
    }
}
