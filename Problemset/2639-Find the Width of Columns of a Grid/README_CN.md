# 2639. 查询网格图中每一列的宽度
给你一个下标从 **0** 开始的 `m x n` 整数矩阵 `grid` 。矩阵中某一列的宽度是这一列数字的最大 **字符串长度** 。

* 比方说，如果 `grid = [[-10], [3], [12]]` ，那么唯一一列的宽度是 `3` ，因为 `-10` 的字符串长度为 `3` 。

请你返回一个大小为 `n` 的整数数组 `ans` ，其中 `ans[i]` 是第 `i` 列的宽度。

一个有 `len` 个数位的整数 `x` ，如果是非负数，那么 **字符串长度** 为 `len` ，否则为 `len + 1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> grid = [[1],[22],[333]]
<strong>输出:</strong> [3]
<strong>解释:</strong> 第 0 列中，333 字符串长度为 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> grid = [[-15,1,3],[15,7,12],[5,6,-2]]
<strong>输出:</strong> [3,1,2]
<strong>解释:</strong>
第 0 列中，只有 -15 字符串长度为 3 。
第 1 列中，所有整数的字符串长度都是 1 。
第 2 列中，12 和 -2 的字符串长度都为 2 。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 100`
* <code>-10<sup>9</sup> <= grid[r][c] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = vec![];

        for i in 0..n {
            let max_num = (0..m).map(|j| grid[j][i]).max().unwrap();
            let min_num = (0..m).map(|j| grid[j][i]).min().unwrap();

            ans.push(max_num.to_string().len().max(min_num.to_string().len()) as i32);
        }

        ans
    }
}
```
