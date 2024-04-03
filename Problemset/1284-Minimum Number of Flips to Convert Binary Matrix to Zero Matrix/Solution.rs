use std::collections::VecDeque;

impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut min_steps = vec![None; 1 << (m * n)];
        let mut deque = VecDeque::new();
        let mut bin_mat = 0;

        for row in 0..m {
            for col in 0..n {
                bin_mat |= (mat[row][col] as usize) << (row * n + col);
            }
        }

        min_steps[bin_mat] = Some(0);
        deque.push_back(bin_mat);

        while let Some(x) = deque.pop_front() {
            if x == 0 {
                break;
            }

            for row in 0..m {
                for col in 0..n {
                    let mut y = x;

                    y ^= 1 << (row * n + col);
                    y ^= ((row > 0) as usize) << (row * n + col - n);
                    y ^= ((row < m - 1) as usize) << (row * n + col + n);
                    y ^= ((col > 0) as usize) << (row * n + col - 1);
                    y ^= ((col < n - 1) as usize) << (row * n + col + 1);

                    if min_steps[y].is_none() {
                        min_steps[y] = Some(min_steps[x].unwrap() + 1);
                        deque.push_back(y);
                    }
                }
            }
        }

        min_steps[0].unwrap_or(-1)
    }
}
