# 2029. 石子游戏 IX
Alice 和 Bob 再次设计了一款新的石子游戏。现有一行 n 个石子，每个石子都有一个关联的数字表示它的价值。给你一个整数数组 `stones` ，其中 `stones[i]` 是第 `i` 个石子的价值。

Alice 和 Bob 轮流进行自己的回合，**Alice** 先手。每一回合，玩家需要从 `stones` 中移除任一石子。

* 如果玩家移除石子后，导致 **所有已移除石子** 的价值 **总和** 可以被 3 整除，那么该玩家就 **输掉游戏** 。
* 如果不满足上一条，且移除后没有任何剩余的石子，那么 Bob 将会直接获胜（即便是在 Alice 的回合）。

假设两位玩家均采用 **最佳** 决策。如果 Alice 获胜，返回 `true` ；如果 Bob 获胜，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> stones = [2,1]
<strong>输出:</strong> true
<strong>解释:</strong> 游戏进行如下：
- 回合 1：Alice 可以移除任意一个石子。
- 回合 2：Bob 移除剩下的石子。
已移除的石子的值总和为 1 + 2 = 3 且可以被 3 整除。因此，Bob 输，Alice 获胜。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> stones = [2]
<strong>输出:</strong> false
<strong>解释:</strong> Alice 会移除唯一一个石子，已移除石子的值总和为 2 。
由于所有石子都已移除，且值总和无法被 3 整除，Bob 获胜。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> stones = [5,1,2,4,3]
<strong>输出:</strong> false
<strong>解释:</strong> Bob 总会获胜。其中一种可能的游戏进行方式如下：
- 回合 1：Alice 可以移除值为 1 的第 2 个石子。已移除石子值总和为 1 。
- 回合 2：Bob 可以移除值为 3 的第 5 个石子。已移除石子值总和为 = 1 + 3 = 4 。
- 回合 3：Alices 可以移除值为 4 的第 4 个石子。已移除石子值总和为 = 1 + 3 + 4 = 8 。
- 回合 4：Bob 可以移除值为 2 的第 3 个石子。已移除石子值总和为 = 1 + 3 + 4 + 2 = 10.
- 回合 5：Alice 可以移除值为 5 的第 1 个石子。已移除石子值总和为 = 1 + 3 + 4 + 2 + 5 = 15.
Alice 输掉游戏，因为已移除石子值总和（15）可以被 3 整除，Bob 获胜。
</pre>

#### 提示:
* <code>1 <= stones.length <= 10<sup>5</sup></code>
* <code>1 <= stones[i] <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut count = [0_i32; 3];

        for &x in &stones {
            count[x as usize % 3] += 1;
        }

        (count[0] % 2 == 0 && count[1].min(count[2]) > 0)
            || (count[0] % 2 == 1 && (count[2] - count[1]).abs() > 2)
    }
}
```
