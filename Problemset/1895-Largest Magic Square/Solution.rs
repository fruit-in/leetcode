impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut prefix_sum_row = vec![vec![0; n + 1]; m];
        let mut prefix_sum_col = vec![vec![0; m + 1]; n];
        let mut ret = 1;

        for i in 0..m {
            for j in 0..n {
                prefix_sum_row[i][j + 1] = prefix_sum_row[i][j] + grid[i][j];
                prefix_sum_col[j][i + 1] = prefix_sum_col[j][i] + grid[i][j];

                for k in (2..=i.min(j) + 1).rev() {
                    if k <= ret {
                        break;
                    }

                    let sum = (0..k).map(|l| grid[i - l][j - l]).sum::<i32>();
                    if (0..k).map(|l| grid[i - l][j + 1 - k + l]).sum::<i32>() != sum {
                        continue;
                    }
                    if (i + 1 - k..=i)
                        .any(|r| prefix_sum_row[r][j + 1] - prefix_sum_row[r][j + 1 - k] != sum)
                    {
                        continue;
                    }
                    if (j + 1 - k..=j)
                        .any(|c| prefix_sum_col[c][i + 1] - prefix_sum_col[c][i + 1 - k] != sum)
                    {
                        continue;
                    }

                    ret = k;
                }
            }
        }

        ret as i32
    }
}
