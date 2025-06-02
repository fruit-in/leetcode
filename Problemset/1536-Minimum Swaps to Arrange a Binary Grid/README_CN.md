# 1536. 排布二进制网格的最少交换次数
给你一个 `n x n` 的二进制网格 `grid`，每一次操作中，你可以选择网格的 **相邻两行** 进行交换。

一个符合要求的网格需要满足主对角线以上的格子全部都是 **0** 。

请你返回使网格满足要求的最少操作次数，如果无法使网格符合要求，请你返回 **-1** 。

主对角线指的是从 `(1, 1)` 到 `(n, n)` 的这些格子。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/07/28/fw.jpg)
<pre>
<strong>输入:</strong> grid = [[0,0,1],[1,1,0],[1,0,0]]
<strong>输出:</strong> 3
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/07/16/e2.jpg)
<pre>
<strong>输入:</strong> grid = [[0,1,1,0],[0,1,1,0],[0,1,1,0],[0,1,1,0]]
<strong>输出:</strong> -1
<strong>解释:</strong> 所有行都是一样的，交换相邻行无法使网格符合要求。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2020/07/16/e3.jpg)
<pre>
<strong>输入:</strong> grid = [[1,0,0],[1,1,0],[1,1,1]]
<strong>输出:</strong> 0
</pre>

#### 提示:
* `n == grid.length`
* `n == grid[i].length`
* `1 <= n <= 200`
* `grid[i][j]` 要么是 `0` 要么是 `1` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut zeros = vec![0; n];
        let mut ret = 0;

        for i in 0..n {
            for j in (0..n).rev() {
                if grid[i][j] == 0 {
                    zeros[i] += 1;
                } else {
                    break;
                }
            }
        }

        for i in 0..n {
            if let Some(j) = zeros.iter().position(|&x| x >= n - 1 - i) {
                zeros.remove(j);
                ret += j;
            } else {
                return -1;
            }
        }

        ret as i32
    }
}
```
