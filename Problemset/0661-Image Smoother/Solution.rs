impl Solution {
    pub fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![0; m[0].len()]; m.len()];

        for i in 0..m.len() {
            for j in 0..m[0].len() {
                let mut cnt = 0;

                for k in i.saturating_sub(1)..=(i + 1).min(m.len() - 1) {
                    for l in j.saturating_sub(1)..=(j + 1).min(m[0].len() - 1) {
                        ret[i][j] += m[k][l];
                        cnt += 1;
                    }
                }

                ret[i][j] /= cnt;
            }
        }

        ret
    }
}
