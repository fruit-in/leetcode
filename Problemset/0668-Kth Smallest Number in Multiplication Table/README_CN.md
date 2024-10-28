# 668. 乘法表中第k小的数
几乎每一个人都用 [乘法表](https://baike.baidu.com/item/%E4%B9%98%E6%B3%95%E8%A1%A8)。但是你能在乘法表中快速找到第 `k` 小的数字吗？

乘法表是大小为 `m x n` 的一个整数矩阵，其中 `mat[i][j] == i * j`（下标从 **1** 开始）。

给你三个整数 `m`、`n` 和 `k`，请你在大小为 `m x n` 的乘法表中，找出并返回第 `k` 小的数字。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/05/02/multtable1-grid.jpg)
<pre>
<strong>输入:</strong> m = 3, n = 3, k = 5
<strong>输出:</strong> 3
<strong>解释:</strong> 第 5 小的数字是 3 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/05/02/multtable2-grid.jpg)
<pre>
<strong>输入:</strong> m = 2, n = 3, k = 6
<strong>输出:</strong> 6
<strong>解释:</strong> 第 6 小的数字是 6 。
</pre>

#### 提示:
* <code>1 <= m, n <= 3 * 10<sup>4</sup></code>
* `1 <= k <= m * n`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let (mut m, mut n) = (m, n);
        let mut lo = 1;
        let mut hi = m * n;
        let mut flag = false;
        let mut count = 0;

        if m > n {
            (m, n) = (n, m);
        }

        while lo < hi {
            let mi = (lo + hi) / 2;
            flag = false;
            count = 0;

            for i in 1..=m {
                count += n.min(mi / i);
                flag |= mi % i == 0 && mi / i <= n;
            }

            if count < k {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }

        if flag {
            hi
        } else if count == k {
            (1..=m)
                .map(|i| n.min(hi / i + 1) * i)
                .filter(|&x| x > hi)
                .min()
                .unwrap()
        } else {
            (1..=m).map(|i| n.min(hi / i) * i).max().unwrap()
        }
    }
}
```
