use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;
        let mut ret = -1;

        for i in 0..n {
            for j in 0..n {
                if grid[i as usize][j as usize] == 1 {
                    continue;
                }

                let mut cells = vec![(i, j, 0)].into_iter().collect::<VecDeque<_>>();
                let mut seen = vec![(i, j)].into_iter().collect::<HashSet<_>>();
                while let Some((x, y, d)) = cells.pop_front() {
                    if grid[x as usize][y as usize] == 1 {
                        ret = ret.max(d);
                        break;
                    }
                    for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let x = x + dx;
                        let y = y + dy;
                        if x >= 0 && x < n && y >= 0 && y < n && !seen.contains(&(x, y)) {
                            cells.push_back((x, y, d + 1));
                            seen.insert((x, y));
                        }
                    }
                }
            }
        }

        ret
    }
}
