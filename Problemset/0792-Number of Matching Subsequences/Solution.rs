impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut indices = vec![vec![]; 26];
        let mut ret = 0;

        for (i, c) in s.bytes().enumerate() {
            indices[(c - b'a') as usize].push(i);
        }

        for word in &words {
            let mut i = 0;
            let mut flag = true;

            for c in word.bytes() {
                match indices[(c - b'a') as usize].binary_search(&i) {
                    Err(j) if j == indices[(c - b'a') as usize].len() => {
                        flag = false;
                        break;
                    }
                    Ok(j) | Err(j) => i = indices[(c - b'a') as usize][j] + 1,
                }
            }

            ret += flag as i32;
        }

        ret
    }
}
