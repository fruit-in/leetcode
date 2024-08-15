use std::collections::HashSet;

impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut island = HashSet::new();
        let mut edges = HashSet::new();
        let mut stack = vec![];
        let mut ret = i32::MAX;

        'outer: for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    island.insert((i, j));
                    stack.push((i, j));
                    break 'outer;
                }
            }
        }

        while let Some((i, j)) = stack.pop() {
            let mut count = 0;

            if i > 0 && grid[i - 1][j] == 1 {
                if !island.contains(&(i - 1, j)) {
                    island.insert((i - 1, j));
                    stack.push((i - 1, j));
                }
                count += 1;
            }
            if i < n - 1 && grid[i + 1][j] == 1 {
                if !island.contains(&(i + 1, j)) {
                    island.insert((i + 1, j));
                    stack.push((i + 1, j));
                }
                count += 1;
            }
            if j > 0 && grid[i][j - 1] == 1 {
                if !island.contains(&(i, j - 1)) {
                    island.insert((i, j - 1));
                    stack.push((i, j - 1));
                }
                count += 1;
            }
            if j < n - 1 && grid[i][j + 1] == 1 {
                if !island.contains(&(i, j + 1)) {
                    island.insert((i, j + 1));
                    stack.push((i, j + 1));
                }
                count += 1;
            }

            if count < 4 {
                edges.insert((i, j));
            }
        }

        for i0 in 0..n {
            for j0 in 0..n {
                if grid[i0][j0] == 0 || island.contains(&(i0, j0)) {
                    continue;
                }

                for &(i1, j1) in edges.iter() {
                    ret =
                        ret.min((i0 as i32 - i1 as i32).abs() + (j0 as i32 - j1 as i32).abs() - 1);
                }
            }
        }

        ret
    }
}
