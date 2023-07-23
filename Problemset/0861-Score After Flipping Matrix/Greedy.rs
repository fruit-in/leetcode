impl Solution {
    pub fn matrix_score(a: Vec<Vec<i32>>) -> i32 {
        let m = a.len();
        let n = a[0].len();
        let mut ret = 0;

        for c in 0..n {
            let mut zeros = 0;

            for r in 0..m {
                zeros += a[r][0] ^ a[r][c];
            }

            ret += zeros.max(m as i32 - zeros) * (1 << (a[0].len() - 1 - c));
        }

        ret
    }
}
