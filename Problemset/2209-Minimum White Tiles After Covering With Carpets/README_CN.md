# 2209. 用地毯覆盖后的最少白色砖块
给你一个下标从 **0** 开始的 **二进制** 字符串 `floor` ，它表示地板上砖块的颜色。

* `floor[i] = '0'` 表示地板上第 `i` 块砖块的颜色是 **黑色** 。
* `floor[i] = '1'` 表示地板上第 `i` 块砖块的颜色是 **白色** 。

同时给你 `numCarpets` 和 `carpetLen` 。你有 `numCarpets` 条 **黑色** 的地毯，每一条 **黑色** 的地毯长度都为 `carpetLen` 块砖块。请你使用这些地毯去覆盖砖块，使得未被覆盖的剩余 **白色** 砖块的数目 **最小** 。地毯相互之间可以覆盖。

请你返回没被覆盖的白色砖块的 **最少** 数目。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/02/10/ex1-1.png)
<pre>
<strong>输入:</strong> floor = "10110101", numCarpets = 2, carpetLen = 2
<strong>输出:</strong> 2
<strong>解释:</strong>
上图展示了剩余 2 块白色砖块的方案。
没有其他方案可以使未被覆盖的白色砖块少于 2 块。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/02/10/ex2.png)
<pre>
<strong>输入:</strong> floor = "11111", numCarpets = 2, carpetLen = 3
<strong>输出:</strong> 0
<strong>解释:</strong>
上图展示了所有白色砖块都被覆盖的一种方案。
注意，地毯相互之间可以覆盖。
</pre>

#### 提示:
* `1 <= carpetLen <= floor.length <= 1000`
* `floor[i]` 要么是 `'0'` ，要么是 `'1'` 。
* `1 <= numCarpets <= 1000`

## 题解 (Rust)

### 1. 题解
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
