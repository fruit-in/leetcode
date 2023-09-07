# 2397. 被列覆盖的最多行数
给你一个下标从 **0** 开始的 `m x n` 二进制矩阵 `mat` 和一个整数 `cols` ，表示你需要选出的列数。

如果一行中，所有的 `1` 都被你选中的列所覆盖，那么我们称这一行 **被覆盖** 了。

请你返回在选择 `cols` 列的情况下，**被覆盖** 的行数 **最大** 为多少。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/07/14/rowscovered.png)
<pre>
<strong>输入:</strong> matrix = [[0,0,0],[1,0,1],[0,1,1],[0,0,1]], numSelect = 2
<strong>输出:</strong> 3
<strong>解释:</strong>
如上图所示，覆盖 3 行的一种可行办法是选择第 0 和第 2 列。
可以看出，不存在大于 3 行被覆盖的方案，所以我们返回 3 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/07/14/rowscovered2.png)
<pre>
<strong>输入:</strong> matrix = [[1],[0]], numSelect = 1
<strong>输出:</strong> 2
<strong>解释:</strong>
选择唯一的一列，两行都被覆盖了，原因是整个矩阵都被覆盖了。
所以我们返回 2 。
</pre>

#### 提示:
* `m == matrix.length`
* `n == matrix[i].length`
* `1 <= m, n <= 12`
* `mat[i][j]` 要么是 `0` 要么是 `1` 。
* `1 <= numSelect <= n`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut comb = (1_usize..2 << n)
            .filter(|x| x.count_ones() as i32 == num_select)
            .collect::<Vec<_>>();
        let mut ret = 0;

        for &s in &comb {
            let mut covered = 0;

            for row in 0..m {
                if (0..n)
                    .filter(|&col| matrix[row][col] == 1)
                    .all(|col| s & (1 << col) != 0)
                {
                    covered += 1;
                }
            }

            ret = ret.max(covered);
        }

        ret
    }
}
```
