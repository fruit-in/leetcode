# 1643. Kth Smallest Instructions
Bob is standing at cell `(0, 0)`, and he wants to reach `destination`: `(row, column)`. He can only travel right and **down**. You are going to help Bob by providing **instructions** for him to reach `destination`.

The **instructions** are represented as a string, where each character is either:

* `'H'`, meaning move horizontally (go **right**), or
* `'V'`, meaning move vertically (go **down**).

Multiple **instructions** will lead Bob to `destination`. For example, if `destination` is `(2, 3)`, both `"HHHVV"` and `"HVHVH"` are valid **instructions**.

However, Bob is very picky. Bob has a lucky number `k`, and he wants the <code>k<sup>th</sup></code> **lexicographically smallest instructions** that will lead him to `destination`. `k` is **1-indexed**.

Given an integer array `destination` and an integer `k`, return *the* <code>k<sup>th</sup></code> ***lexicographically smallest instructions** that will take Bob to* `destination`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/10/12/ex1.png)
<pre>
<strong>Input:</strong> destination = [2,3], k = 1
<strong>Output:</strong> "HHHVV"
<strong>Explanation:</strong> All the instructions that reach (2, 3) in lexicographic order are as follows:
["HHHVV", "HHVHV", "HHVVH", "HVHHV", "HVHVH", "HVVHH", "VHHHV", "VHHVH", "VHVHH", "VVHHH"].
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/10/12/ex2.png)
<pre>
<strong>Input:</strong> destination = [2,3], k = 2
<strong>Output:</strong> "HHVHV"
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2020/10/12/ex3.png)
<pre>
<strong>Input:</strong> destination = [2,3], k = 3
<strong>Output:</strong> "HHVVH"
</pre>

#### Constraints:
* `destination.length == 2`
* `1 <= row, column <= 15`
* `1 <= k <= nCr(row + column, row)`, where `nCr(a, b)` denotes `a` choose `b`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let row = destination[0] as usize;
        let column = destination[1] as usize;
        let mut k = k as usize;
        let mut dp = vec![vec![1; row + 1]; row + column];
        let mut remainv = row;
        let mut remainh = column;
        let mut instructions = vec![];

        for n in 0..dp.len() {
            for m in 1..dp[0].len().min(n + 1) {
                dp[n][m] = dp[n][m - 1] * (n - m + 1) / m;
            }
        }

        for _ in 0..row + column {
            if remainh > 0 && k <= dp[remainv + remainh - 1][remainv] {
                remainh -= 1;
                instructions.push(b'H');
            } else {
                k -= dp[remainv + remainh - 1][remainv];
                remainv -= 1;
                instructions.push(b'V');
            }
        }

        String::from_utf8(instructions).unwrap()
    }
}
```
