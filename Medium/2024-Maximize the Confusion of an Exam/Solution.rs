impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let answer_key = answer_key.as_bytes();
        let mut ret = 0;

        for ch in [b'T', b'F'] {
            let mut i = 0;
            let mut count = 0;

            for j in 0..answer_key.len() {
                count += (answer_key[j] == ch) as i32;
                while count > k {
                    count -= (answer_key[i] == ch) as i32;
                    i += 1;
                }
                ret = ret.max(j - i + 1);
            }
        }

        ret as i32
    }
}
