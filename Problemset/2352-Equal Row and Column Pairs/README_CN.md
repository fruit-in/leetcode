# 2352. 相等行列对
给你一个下标从 **0** 开始、大小为 `n x n` 的整数矩阵 `grid` ，返回满足 <code>R<sub>i</sub></code> 行和 <code>C<sub>j</sub></code> 列相等的行列对 <code>(R<sub>i</sub>, C<sub>j</sub>)</code> 的数目。

如果行和列以相同的顺序包含相同的元素（即相等的数组），则认为二者是相等的。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/06/01/ex1.jpg)
<pre>
<strong>输入:</strong> grid = [[3,2,1],[1,7,6],[2,7,7]]
<strong>输出:</strong> 1
<strong>解释:</strong> 存在一对相等行列对：
- (第 2 行，第 1 列)：[2,7,7]
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/06/01/ex2.jpg)
<pre>
<strong>输入:</strong> grid = [[3,1,2,2],[1,4,4,5],[2,4,2,2],[2,4,2,2]]
<strong>输出:</strong> 3
<strong>解释:</strong> 存在三对相等行列对：
- (第 0 行，第 0 列)：[3,1,2,2]
- (第 2 行, 第 2 列)：[2,4,2,2]
- (第 3 行, 第 2 列)：[2,4,2,2]
</pre>

#### 提示:
* `n == grid.length == grid[i].length`
* `1 <= n <= 200`
* <code>1 <= grid[i][j] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for i in 0..grid.len() {
            count
                .entry(grid[i].clone())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        for i in 0..grid.len() {
            let col = (0..grid.len()).map(|j| grid[j][i]).collect::<Vec<_>>();

            ret += count.get(&col).unwrap_or(&0);
        }

        ret
    }
}
```
