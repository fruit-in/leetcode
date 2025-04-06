impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let s = s.as_bytes();
        let mut indices = vec![vec![-1]; 26];
        let mut ret = 0;

        for i in 0..s.len() {
            indices[(s[i] - b'A') as usize].push(i as i32);
        }

        for i in 0..26 {
            indices[i].push(s.len() as i32);

            for j in 1..indices[i].len() - 1 {
                ret += (indices[i][j] - indices[i][j - 1]) * (indices[i][j + 1] - indices[i][j]);
            }
        }

        ret
    }
}
