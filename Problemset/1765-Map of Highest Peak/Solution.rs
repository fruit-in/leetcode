use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut is_water = is_water;
        let m = is_water.len();
        let n = is_water[0].len();
        let mut deque = VecDeque::new();
        let mut ret = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    deque.push_back((i, j));
                }
            }
        }

        while let Some((i, j)) = deque.pop_front() {
            if i > 0 && is_water[i - 1][j] == 0 {
                is_water[i - 1][j] = 1;
                deque.push_back((i - 1, j));
                ret[i - 1][j] = ret[i][j] + 1;
            }
            if i + 1 < m && is_water[i + 1][j] == 0 {
                is_water[i + 1][j] = 1;
                deque.push_back((i + 1, j));
                ret[i + 1][j] = ret[i][j] + 1;
            }
            if j > 0 && is_water[i][j - 1] == 0 {
                is_water[i][j - 1] = 1;
                deque.push_back((i, j - 1));
                ret[i][j - 1] = ret[i][j] + 1;
            }
            if j + 1 < n && is_water[i][j + 1] == 0 {
                is_water[i][j + 1] = 1;
                deque.push_back((i, j + 1));
                ret[i][j + 1] = ret[i][j] + 1;
            }
        }

        ret
    }
}
