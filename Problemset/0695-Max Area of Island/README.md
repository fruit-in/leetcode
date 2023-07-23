# 695. Max Area of Island
You are given an `m x n` binary matrix `grid`. An island is a group of `1`'s (representing land) connected **4-directionally** (horizontal or vertical.) You may assume all four edges of the grid are surrounded by water.

The **area** of an island is the number of cells with a value `1` in the island.

Return *the maximum **area** of an island in* `grid`. If there is no island, return `0`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/05/01/maxarea1-grid.jpg)
<pre>
<strong>Input:</strong> grid = [[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The answer is not 11, because the island must be connected 4-directionally.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> grid = [[0,0,0,0,0,0,0,0]]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 50`
* `grid[i][j]` is either `0` or `1`.

## Solutions (Ruby)

### 1. DFS
```Ruby
# @param {Integer[][]} grid
# @return {Integer}
def max_area_of_island(grid)
  m = grid.size
  n = grid[0].size
  ret = 0

  (0...m).each do |i|
    (0...n).each do |j|
      next if grid[i][j] == 0

      area = 0
      cells = [[i, j]]

      until cells.empty?
        i, j = cells.pop
        next if grid[i][j] == 0

        area += 1
        grid[i][j] = 0

        cells.push([i - 1, j]) if i > 0 && grid[i - 1][j] == 1
        cells.push([i + 1, j]) if i < m - 1 && grid[i + 1][j] == 1
        cells.push([i, j - 1]) if j > 0 && grid[i][j - 1] == 1
        cells.push([i, j + 1]) if j < n - 1 && grid[i][j + 1] == 1
      end

      ret = [ret, area].max
    end
  end

  ret
end
```

## Solutions (Rust)

### 1. DFS
```Rust
impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    continue;
                }

                let mut area = 0;
                let mut cells = vec![(i, j)];

                while let Some((i, j)) = cells.pop() {
                    if grid[i][j] == 0 {
                        continue;
                    }

                    area += 1;
                    grid[i][j] = 0;

                    if i > 0 && grid[i - 1][j] == 1 {
                        cells.push((i - 1, j));
                    }
                    if i < m - 1 && grid[i + 1][j] == 1 {
                        cells.push((i + 1, j));
                    }
                    if j > 0 && grid[i][j - 1] == 1 {
                        cells.push((i, j - 1));
                    }
                    if j < n - 1 && grid[i][j + 1] == 1 {
                        cells.push((i, j + 1));
                    }
                }

                ret = ret.max(area);
            }
        }

        ret
    }
}
```
