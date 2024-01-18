# 526. 优美的排列
假设有从 1 到 n 的 n 个整数。用这些整数构造一个数组 `perm`（**下标从 1 开始**），只要满足下述条件 **之一** ，该数组就是一个 **优美的排列** ：

* `perm[i]` 能够被 `i` 整除
* `i` 能够被 `perm[i]` 整除

给你一个整数 `n` ，返回可以构造的 **优美排列** 的 **数量** 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> 2
<strong>解释:</strong>
第 1 个优美的排列是 [1,2]：
    - perm[1] = 1 能被 i = 1 整除
    - perm[2] = 2 能被 i = 2 整除
第 2 个优美的排列是 [2,1]:
    - perm[1] = 2 能被 i = 1 整除
    - i = 2 能被 perm[2] = 1 整除
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= n <= 15`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        Self::backtracking(n, 1, 0)
    }

    fn backtracking(n: i32, i: i32, bitmask: i32) -> i32 {
        if i > n {
            return 1;
        }

        (1..=n)
            .filter(|p| (bitmask >> p) & 1 == 0 && (p % i == 0 || i % p == 0))
            .map(|p| Self::backtracking(n, i + 1, bitmask | (1 << p)))
            .sum()
    }
}
```
