# 668. Kth Smallest Number in Multiplication Table
Nearly everyone has used the [Multiplication Table](https://en.wikipedia.org/wiki/Multiplication_table). The multiplication table of size `m x n` is an integer matrix `mat` where `mat[i][j] == i * j` (**1-indexed**).

Given three integers `m`, `n`, and `k`, return *the* <code>k<sup>th</sup></code> *smallest element in the* `m x n` *multiplication table*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/05/02/multtable1-grid.jpg)
<pre>
<strong>Input:</strong> m = 3, n = 3, k = 5
<strong>Output:</strong> 3
<strong>Explanation:</strong> The 5th smallest number is 3.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/05/02/multtable2-grid.jpg)
<pre>
<strong>Input:</strong> m = 2, n = 3, k = 6
<strong>Output:</strong> 6
<strong>Explanation:</strong> The 6th smallest number is 6.
</pre>

#### Constraints:
* <code>1 <= m, n <= 3 * 10<sup>4</sup></code>
* `1 <= k <= m * n`

## Solutions (Rust)

### 1. Solution
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
