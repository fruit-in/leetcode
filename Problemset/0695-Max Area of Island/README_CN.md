# 695. 岛屿的最大面积
给定一个包含了一些 `0` 和 `1` 的非空二维数组 `grid` 。

一个 **岛屿** 是由一些相邻的 `1` (代表土地) 构成的组合，这里的「相邻」要求两个 `1` 必须在水平或者竖直方向上相邻。你可以假设 `grid` 的四个边缘都被 `0`（代表水）包围着。

找到给定的二维数组中最大的岛屿面积。(如果没有岛屿，则返回面积为 `0` 。)

#### 示例 1:
<pre>
[[0,0,1,0,0,0,0,1,0,0,0,0,0],
 [0,0,0,0,0,0,0,1,1,1,0,0,0],
 [0,1,1,0,1,0,0,0,0,0,0,0,0],
 [0,1,0,0,1,1,0,0,<b>1</b>,0,<b>1</b>,0,0],
 [0,1,0,0,1,1,0,0,<b>1</b>,<b>1</b>,<b>1</b>,0,0],
 [0,0,0,0,0,0,0,0,0,0,<b>1</b>,0,0],
 [0,0,0,0,0,0,0,1,1,1,0,0,0],
 [0,0,0,0,0,0,0,1,1,0,0,0,0]]
</pre>

对于上面这个给定矩阵应返回 `6`。注意答案不应该是 `11` ，因为岛屿只能包含水平或垂直的四个方向的 `1` 。

#### 示例 2:
```
[[0,0,0,0,0,0,0,0]]
```

对于上面这个给定的矩阵, 返回 `0`。

**注意:** 给定的矩阵`grid` 的长度和宽度都不超过 50。

## 题解 (Ruby)

### 1. 深度优先搜索
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

## 题解 (Rust)

### 1. 深度优先搜索
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
