# 840. 矩阵中的幻方
3 x 3 的幻方是一个填充有**从 1 到 9** 的不同数字的 3 x 3 矩阵，其中每行，每列以及两条对角线上的各数之和都相等。

给定一个由整数组成的 ```grid```，其中有多少个 3 × 3 的 “幻方” 子矩阵？（每个子矩阵都是连续的）。

#### 示例:
<pre>
<strong>输入:</strong> [[4,3,8,4],
      [9,5,1,9],
      [2,7,6,2]]
<strong>输出:</strong> 1
<strong>解释:</strong>
下面的子矩阵是一个 3 x 3 的幻方：
438
951
276

而这一个不是：
384
519
762

总的来说，在本示例所给定的矩阵中只有一个 3 x 3 的幻方子矩阵。
</pre>

#### 提示:
1. ```1 <= grid.length <= 10```
2. ```1 <= grid[0].length <= 10```
3. ```0 <= grid[i][j] <= 15```

## 题解 (Rust)

### 1. 暴力法
```Rust
impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        for i in 2..grid.len() {
            for j in 2..grid[0].len() {
                if grid[i - 1][j - 1] == 5 &&
                   grid[i - 2][j - 2] + grid[i][j] == 10 &&
                   grid[i - 2][j] + grid[i][j - 2] == 10 &&
                   grid[i - 1][j - 2] + grid[i - 1][j] == 10 &&
                   grid[i - 2][j - 1] + grid[i][j - 1] == 10 &&
                   grid[i - 2][j - 2] + grid[i - 2][j - 1] + grid[i - 2][j] == 15 &&
                   grid[i - 2][j - 2] + grid[i - 1][j - 2] + grid[i][j - 2] == 15 {

                    let mut nums = Vec::new();
                    nums.extend_from_slice(&grid[i - 2][(j - 2)..(j + 1)]);
                    nums.extend_from_slice(&grid[i - 1][(j - 2)..(j + 1)]);
                    nums.extend_from_slice(&grid[i][(j - 2)..(j + 1)]);
                    nums.sort_unstable();

                    if nums == vec![1, 2, 3, 4, 5, 6, 7, 8, 9] {
                        ret += 1;
                    }
                }
            }
        }

        ret
    }
}
```
