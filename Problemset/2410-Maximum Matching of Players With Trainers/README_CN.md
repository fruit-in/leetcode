# 2410. 运动员和训练师的最大匹配数
给你一个下标从 **0** 开始的整数数组 `players` ，其中 `players[i]` 表示第 `i` 名运动员的 **能力** 值，同时给你一个下标从 **0** 开始的整数数组 `trainers` ，其中 `trainers[j]` 表示第 `j` 名训练师的 **训练能力值** 。

如果第 `i` 名运动员的能力值 **小于等于** 第 `j` 名训练师的能力值，那么第 `i` 名运动员可以 **匹配** 第 `j` 名训练师。除此以外，每名运动员至多可以匹配一位训练师，每位训练师最多可以匹配一位运动员。

请你返回满足上述要求 `players` 和 `trainers` 的 **最大** 匹配数。

#### 示例 1:
<pre>
<strong>输入:</strong> players = [4,7,9], trainers = [8,2,5,8]
<strong>输出:</strong> 2
<strong>解释:</strong>
得到两个匹配的一种方案是：
- players[0] 与 trainers[0] 匹配，因为 4 <= 8 。
- players[1] 与 trainers[3] 匹配，因为 7 <= 8 。
可以证明 2 是可以形成的最大匹配数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> players = [1,1,1], trainers = [10]
<strong>输出:</strong> 1
<strong>解释:</strong>
训练师可以匹配所有 3 个运动员
每个运动员至多只能匹配一个训练师，所以最大答案是 1 。
</pre>

#### 提示:
* <code>1 <= players.length, trainers.length <= 10<sup>5</sup></code>
* <code>1 <= players[i], trainers[j] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut players = players;
        let mut trainers = trainers;
        let mut ret = 0;

        players.sort_unstable();
        trainers.sort_unstable();

        while let Some(trainer) = trainers.pop() {
            while *players.last().unwrap_or(&0) > trainer {
                players.pop();
            }
            if players.pop().unwrap_or(i32::MAX) <= trainer {
                ret += 1;
            }
        }

        ret
    }
}
```
