# 1411. 给 N x 3 网格图涂色的方案数
你有一个 `n x 3` 的网格图 `grid` ，你需要用 **红，黄，绿** 三种颜色之一给每一个格子上色，且确保相邻格子颜色不同（也就是有相同水平边或者垂直边的格子颜色不同）。

给你网格图的行数 `n` 。

请你返回给 `grid` 涂色的方案数。由于答案可能会非常大，请你返回答案对 `10^9 + 7` 取余的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 12
<strong>解释:</strong> 总共有 12 种可行的方法：
<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/04/12/e1.png">
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> 54
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> 246
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> n = 7
<strong>输出:</strong> 106494
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> n = 5000
<strong>输出:</strong> 30228214
</pre>

#### 提示:
* `n == grid.length`
* `grid[i].length == 3`
* `1 <= n <= 5000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut x = 6;
        let mut y = 6;

        for _ in 1..n {
            x = (x + y) % 1_000_000_007 * 2 % 1_000_000_007;
            y = (x + y) % 1_000_000_007;
        }

        (x + y) % 1_000_000_007
    }
}
```
