impl Solution {
    pub fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut count25 = vec![vec![(0, 0); n]; m];
        let mut psud = count25.clone();
        let mut pslr = count25.clone();
        let mut psdu = count25.clone();
        let mut psrl = count25.clone();
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                let mut x = grid[i][j];

                while x % 2 == 0 {
                    count25[i][j].0 += 1;
                    x /= 2;
                }
                while x % 5 == 0 {
                    count25[i][j].1 += 1;
                    x /= 5;
                }

                psud[i][j] = count25[i][j];
                if i > 0 {
                    psud[i][j].0 += psud[i - 1][j].0;
                    psud[i][j].1 += psud[i - 1][j].1;
                }
                pslr[i][j] = count25[i][j];
                if j > 0 {
                    pslr[i][j].0 += pslr[i][j - 1].0;
                    pslr[i][j].1 += pslr[i][j - 1].1;
                }
            }
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                psdu[i][j] = count25[i][j];
                if i < m - 1 {
                    psdu[i][j].0 += psdu[i + 1][j].0;
                    psdu[i][j].1 += psdu[i + 1][j].1;
                }
                psrl[i][j] = count25[i][j];
                if j < n - 1 {
                    psrl[i][j].0 += psrl[i][j + 1].0;
                    psrl[i][j].1 += psrl[i][j + 1].1;
                }

                ret = ret
                    .max(
                        (psud[i][j].0 + pslr[i][j].0 - count25[i][j].0)
                            .min(psud[i][j].1 + pslr[i][j].1 - count25[i][j].1),
                    )
                    .max(
                        (psud[i][j].0 + psrl[i][j].0 - count25[i][j].0)
                            .min(psud[i][j].1 + psrl[i][j].1 - count25[i][j].1),
                    )
                    .max(
                        (psdu[i][j].0 + pslr[i][j].0 - count25[i][j].0)
                            .min(psdu[i][j].1 + pslr[i][j].1 - count25[i][j].1),
                    )
                    .max(
                        (psdu[i][j].0 + psrl[i][j].0 - count25[i][j].0)
                            .min(psdu[i][j].1 + psrl[i][j].1 - count25[i][j].1),
                    );
            }
        }

        ret
    }
}
