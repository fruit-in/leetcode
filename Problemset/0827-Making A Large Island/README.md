# 827. Making A Large Island
You are given an `n x n` binary matrix `grid`. You are allowed to change **at most one** `0` to be `1`.

Return *the size of the largest **island** in* `grid` *after applying this operation*.

An **island** is a 4-directionally connected group of `1`s.

#### Example 1:
<pre>
<strong>Input:</strong> grid = [[1,0],[0,1]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Change one 0 to 1 and connect two 1s, then we get an island with area = 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> grid = [[1,1],[1,0]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Change the 0 to 1 and make the island bigger, only one island with area = 4.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> grid = [[1,1],[1,1]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Can't change any 0 to 1, only one island with area = 4.
</pre>

#### Constraints:
* `n == grid.length`
* `n == grid[i].length`
* `1 <= n <= 500`
* `grid[i][j]` is either `0` or `1`.

## Solutions (Rust)

### 1. Solution
```Rust
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
```
