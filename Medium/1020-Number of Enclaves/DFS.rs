use std::collections::HashSet;

impl Solution {
    pub fn num_enclaves(a: Vec<Vec<i32>>) -> i32 {
        let m = a.len() as i32;
        let n = a[0].len() as i32;
        let mut seen = HashSet::new();
        let mut cells = Vec::new();
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                if a[i as usize][j as usize] == 1 {
                    if i == 0 || j == 0 || i == m - 1 || j == n - 1 {
                        seen.insert((i, j));
                        cells.push((i, j));
                    } else {
                        ret += 1;
                    }
                }
            }
        }

        while let Some((i, j)) = cells.pop() {
            for (x, y) in &[(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let (i, j) = (i + x, j + y);
                if i > 0
                    && i < m - 1
                    && j > 0
                    && j < n - 1
                    && a[i as usize][j as usize] == 1
                    && !seen.contains(&(i, j))
                {
                    seen.insert((i, j));
                    cells.push((i, j));
                    ret -= 1;
                }
            }
        }

        ret
    }
}
