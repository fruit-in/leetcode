# 403. 青蛙过河
一只青蛙想要过河。 假定河流被等分为若干个单元格，并且在每一个单元格内都有可能放有一块石子（也有可能没有）。 青蛙可以跳上石子，但是不可以跳入水中。

给你石子的位置列表 `stones`（用单元格序号 **升序** 表示）， 请判定青蛙能否成功过河（即能否在最后一步跳至最后一块石子上）。开始时， 青蛙默认已站在第一块石子上，并可以假定它第一步只能跳跃 `1` 个单位（即只能从单元格 1 跳至单元格 2 ）。

如果青蛙上一步跳跃了 `k` 个单位，那么它接下来的跳跃距离只能选择为 `k - 1`、`k` 或 `k + 1` 个单位。 另请注意，青蛙只能向前方（终点的方向）跳跃。

#### 示例 1:
<pre>
<strong>输入:</strong> stones = [0,1,3,5,6,8,12,17]
<strong>输出:</strong> true
<strong>解释:</strong> 青蛙可以成功过河，按照如下方案跳跃：跳 1 个单位到第 2 块石子, 然后跳 2 个单位到第 3 块石子, 接着 跳 2 个单位到第 4 块石子, 然后跳 3 个单位到第 6 块石子, 跳 4 个单位到第 7 块石子, 最后，跳 5 个单位到第 8 个石子（即最后一块石子）。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> stones = [0,1,2,3,4,8,9,11]
<strong>输出:</strong> false
<strong>解释:</strong> 这是因为第 5 和第 6 个石子之间的间距太大，没有可选的方案供青蛙跳跃过去。
</pre>

#### 提示:
* `2 <= stones.length <= 2000`
* <code>0 <= stones[i] <= 2<sup>31</sup> - 1</code>
* `stones[0] == 0`
* `stones` 按严格升序排列

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        if stones[1] != 1 {
            return false;
        }

        let stones = stones.into_iter().map(|s| s as usize).collect::<Vec<_>>();
        let mut indices = stones
            .iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect::<HashMap<_, _>>();
        let mut dp = vec![vec![false; stones.len()]; stones.len()];
        dp[1][1] = true;

        for i in 1..stones.len() {
            for k in 1..=i {
                if dp[i][k] {
                    if i == stones.len() - 1 {
                        return true;
                    }

                    if let Some(&j) = indices.get(&(stones[i] + k - 1)) {
                        dp[j][k - 1] = true;
                    }
                    if let Some(&j) = indices.get(&(stones[i] + k)) {
                        dp[j][k] = true;
                    }
                    if let Some(&j) = indices.get(&(stones[i] + k + 1)) {
                        dp[j][k + 1] = true;
                    }
                }
            }
        }

        false
    }
}
```
