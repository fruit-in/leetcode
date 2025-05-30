impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let n = grid.len();
        let mut parent = HashMap::new();
        let mut count = HashMap::new();
        let mut ret = 0;

        for r in 0..n {
            for c in 0..n {
                if grid[r][c] == 0 || parent.contains_key(&(r, c)) {
                    continue;
                }

                let mut stack = vec![(r, c)];
                parent.insert((r, c), (r, c));
                count.insert((r, c), 0);

                while let Some((i, j)) = stack.pop() {
                    *count.get_mut(&(r, c)).unwrap() += 1;

                    if i > 0 && grid[i - 1][j] == 1 && !parent.contains_key(&(i - 1, j)) {
                        stack.push((i - 1, j));
                        parent.insert((i - 1, j), (r, c));
                    }
                    if i < n - 1 && grid[i + 1][j] == 1 && !parent.contains_key(&(i + 1, j)) {
                        stack.push((i + 1, j));
                        parent.insert((i + 1, j), (r, c));
                    }
                    if j > 0 && grid[i][j - 1] == 1 && !parent.contains_key(&(i, j - 1)) {
                        stack.push((i, j - 1));
                        parent.insert((i, j - 1), (r, c));
                    }
                    if j < n - 1 && grid[i][j + 1] == 1 && !parent.contains_key(&(i, j + 1)) {
                        stack.push((i, j + 1));
                        parent.insert((i, j + 1), (r, c));
                    }
                }

                ret = ret.max(count[&(r, c)]);
            }
        }

        for r in 0..n {
            for c in 0..n {
                if grid[r][c] == 0 {
                    let mut islands = HashSet::new();

                    if r > 0 && grid[r - 1][c] == 1 {
                        islands.insert(parent[&(r - 1, c)]);
                    }
                    if r < n - 1 && grid[r + 1][c] == 1 {
                        islands.insert(parent[&(r + 1, c)]);
                    }
                    if c > 0 && grid[r][c - 1] == 1 {
                        islands.insert(parent[&(r, c - 1)]);
                    }
                    if c < n - 1 && grid[r][c + 1] == 1 {
                        islands.insert(parent[&(r, c + 1)]);
                    }

                    ret = ret.max(1 + islands.iter().map(|&(r, c)| count[&(r, c)]).sum::<i32>());
                }
            }
        }

        ret
    }
}
