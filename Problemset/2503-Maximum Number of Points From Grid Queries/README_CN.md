# 2503. 矩阵查询可获得的最大分数
给你一个大小为 `m x n` 的整数矩阵 `grid` 和一个大小为 `k` 的数组 `queries` 。

找出一个大小为 `k` 的数组 `answer` ，且满足对于每个整数 `queries[i]` ，你从矩阵 **左上角** 单元格开始，重复以下过程：
* 如果 `queries[i]` **严格** 大于你当前所处位置单元格，如果该单元格是第一次访问，则获得 1 分，并且你可以移动到所有 `4` 个方向（上、下、左、右）上任一 **相邻** 单元格。
* 否则，你不能获得任何分，并且结束这一过程。

在过程结束后，`answer[i]` 是你可以获得的最大分数。注意，对于每个查询，你可以访问同一个单元格 **多次** 。

返回结果数组 `answer` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2025/03/15/image1.png)
<pre>
<strong>输入:</strong> grid = [[1,2,3],[2,5,7],[3,5,1]], queries = [5,6,2]
<strong>输出:</strong> [5,8,1]
<strong>解释:</strong> 上图展示了每个查询中访问并获得分数的单元格。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/10/20/yetgriddrawio-2.png)
<pre>
<strong>输入:</strong> grid = [[5,2,1],[1,1,2]], queries = [3]
<strong>输出:</strong> [0]
<strong>解释:</strong> 无法获得分数，因为左上角单元格的值大于等于 3 。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `2 <= m, n <= 1000`
* <code>4 <= m * n <= 10<sup>5</sup></code>
* `k == queries.length`
* <code>1 <= k <= 10<sup>4</sup></code>
* <code>1 <= grid[i][j], queries[i] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        use std::collections::BinaryHeap;
        use std::collections::HashSet;

        let (m, n) = (grid.len(), grid[0].len());
        let mut heap = BinaryHeap::from([(-grid[0][0], 0, 0)]);
        let mut visited = HashSet::from([(0, 0)]);
        let mut indices = (0..queries.len()).collect::<Vec<_>>();
        let mut points = 0;
        let mut answer = vec![0; queries.len()];

        indices.sort_unstable_by_key(|&i| queries[i]);

        for &i in &indices {
            while queries[i] > -heap.peek().unwrap_or(&(-1000000, 0, 0)).0 {
                let (_, i, j) = heap.pop().unwrap();

                if i > 0 && !visited.contains(&(i - 1, j)) {
                    heap.push((-grid[i - 1][j], i - 1, j));
                    visited.insert((i - 1, j));
                }
                if i < m - 1 && !visited.contains(&(i + 1, j)) {
                    heap.push((-grid[i + 1][j], i + 1, j));
                    visited.insert((i + 1, j));
                }
                if j > 0 && !visited.contains(&(i, j - 1)) {
                    heap.push((-grid[i][j - 1], i, j - 1));
                    visited.insert((i, j - 1));
                }
                if j < n - 1 && !visited.contains(&(i, j + 1)) {
                    heap.push((-grid[i][j + 1], i, j + 1));
                    visited.insert((i, j + 1));
                }

                points += 1;
            }

            answer[i] = points;
        }

        answer
    }
}
```
