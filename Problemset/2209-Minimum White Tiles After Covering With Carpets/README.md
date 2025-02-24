# 2209. Minimum White Tiles After Covering With Carpets
You are given a **0-indexed** binary string `floor`, which represents the colors of tiles on a floor:

* `floor[i] = '0'` denotes that the <code>i<sup>th</sup></code> tile of the floor is colored **black**.
* On the other hand, `floor[i] = '1'` denotes that the <code>i<sup>th</sup></code> tile of the floor is colored **white**.

You are also given `numCarpets` and `carpetLen`. You have `numCarpets` **black** carpets, each of length `carpetLen` tiles. Cover the tiles with the given carpets such that the number of **white** tiles still visible is **minimum**. Carpets may overlap one another.

Return *the **minimum** number of white tiles still visible*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/02/10/ex1-1.png)
<pre>
<strong>Input:</strong> floor = "10110101", numCarpets = 2, carpetLen = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong>
The figure above shows one way of covering the tiles with the carpets such that only 2 white tiles are visible.
No other way of covering the tiles with the carpets can leave less than 2 white tiles visible.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/02/10/ex2.png)
<pre>
<strong>Input:</strong> floor = "11111", numCarpets = 2, carpetLen = 3
<strong>Output:</strong> 0
<strong>Explanation:</strong>
The figure above shows one way of covering the tiles with the carpets such that no white tiles are visible.
Note that the carpets are able to overlap one another.
</pre>

#### Constraints:
* `1 <= carpetLen <= floor.length <= 1000`
* `floor[i]` is either `'0'` or `'1'`.
* `1 <= numCarpets <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        let num_carpets = num_carpets as usize;
        let carpet_len = carpet_len as usize;
        let floor = floor.as_bytes();
        let mut prefix_sum = vec![0; floor.len() + 1];
        let mut dp = vec![vec![i32::MAX; num_carpets + 1]; floor.len() + 1];

        for i in 0..floor.len() {
            prefix_sum[i + 1] = prefix_sum[i] + (floor[i] - b'0') as i32;
        }

        for i in 0..floor.len() {
            dp[i][0] = *prefix_sum.last().unwrap();
            for j in 0..num_carpets {
                let k = floor.len().min(i + carpet_len);
                dp[k][j + 1] = dp[k][j + 1].min(dp[i][j] - prefix_sum[k] + prefix_sum[i]);
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            }
            dp[i + 1][num_carpets] = dp[i + 1][num_carpets].min(dp[i][num_carpets]);
        }

        *dp[floor.len()].iter().min().unwrap()
    }
}
```
