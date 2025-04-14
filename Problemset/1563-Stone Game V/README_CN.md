# 1563. 石子游戏 V
几块石子 **排成一行** ，每块石子都有一个关联值，关联值为整数，由数组 `stoneValue` 给出。

游戏中的每一轮：Alice 会将这行石子分成两个 **非空行**（即，左侧行和右侧行）；Bob 负责计算每一行的值，即此行中所有石子的值的总和。Bob 会丢弃值最大的行，Alice 的得分为剩下那行的值（每轮累加）。如果两行的值相等，Bob 让 Alice 决定丢弃哪一行。下一轮从剩下的那一行开始。

只 **剩下一块石子** 时，游戏结束。Alice 的分数最初为 `0` 。

返回 **Alice 能够获得的最大分数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> stoneValue = [6,2,3,4,5,5]
<strong>输出:</strong> 18
<strong>解释:</strong> 在第一轮中，Alice 将行划分为 [6，2，3]，[4，5，5] 。左行的值是 11 ，右行的值是 14 。Bob 丢弃了右行，Alice 的分数现在是 11 。
在第二轮中，Alice 将行分成 [6]，[2，3] 。这一次 Bob 扔掉了左行，Alice 的分数变成了 16（11 + 5）。
最后一轮 Alice 只能将行分成 [2]，[3] 。Bob 扔掉右行，Alice 的分数现在是 18（16 + 2）。游戏结束，因为这行只剩下一块石头了。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> stoneValue = [7,7,7,7,7,7,7]
<strong>输出:</strong> 28
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> stoneValue = [4]
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= stoneValue.length <= 500`
* <code>1 <= stoneValue[i] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut dp = vec![vec![0; n]; n];

        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stone_value[i];
        }

        for i in 2..=n {
            for j in 0..n - i + 1 {
                let mut left_sum = 0;
                let mut right_sum = prefix_sum[j + i] - prefix_sum[j];

                for k in j..j + i - 1 {
                    left_sum += stone_value[k];
                    right_sum -= stone_value[k];

                    if left_sum <= right_sum {
                        dp[j][j + i - 1] = dp[j][j + i - 1].max(left_sum + dp[j][k]);
                    }
                    if left_sum >= right_sum {
                        dp[j][j + i - 1] = dp[j][j + i - 1].max(right_sum + dp[k + 1][j + i - 1]);
                    }
                }
            }
        }

        dp[0][n - 1]
    }
}
```
