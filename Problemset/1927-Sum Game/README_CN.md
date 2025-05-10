# 1927. 求和游戏
Alice 和 Bob 玩一个游戏，两人轮流行动，**Alice 先手** 。

给你一个 **偶数长度** 的字符串 `num` ，每一个字符为数字字符或者 `'?'` 。每一次操作中，如果 `num` 中至少有一个 `'?'` ，那么玩家可以执行以下操作：
1. 选择一个下标 `i` 满足 `num[i] == '?'` 。
2. 将 `num[i]` 用 `'0'` 到 `'9'` 之间的一个数字字符替代。

当 `num` 中没有 `'?'` 时，游戏结束。

Bob 获胜的条件是 `num` 中前一半数字的和 **等于** 后一半数字的和。Alice 获胜的条件是前一半的和与后一半的和 **不相等** 。

* 比方说，游戏结束时 `num = "243801"` ，那么 Bob 获胜，因为 `2+4+3 = 8+0+1` 。如果游戏结束时 `num = "243803"` ，那么 Alice 获胜，因为 `2+4+3 != 8+0+3` 。

在 Alice 和 Bob 都采取 **最优** 策略的前提下，如果 Alice 获胜，请返回 `true` ，如果 Bob 获胜，请返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> num = "5023"
<strong>输出:</strong> false
<strong>解释:</strong> num 中没有 '?' ，没法进行任何操作。
前一半的和等于后一半的和：5 + 0 = 2 + 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = "25??"
<strong>输出:</strong> true
<strong>解释:</strong> Alice 可以将两个 '?' 中的一个替换为 '9' ，Bob 无论如何都无法使前一半的和等于后一半的和。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> num = "?3295???"
<strong>输出:</strong> false
<strong>解释:</strong> Bob 总是能赢。一种可能的结果是：
- Alice 将第一个 '?' 用 '9' 替换。num = "93295???" 。
- Bob 将后面一半中的一个 '?' 替换为 '9' 。num = "932959??" 。
- Alice 将后面一半中的一个 '?' 替换为 '2' 。num = "9329592?" 。
- Bob 将后面一半中最后一个 '?' 替换为 '7' 。num = "93295927" 。
Bob 获胜，因为 9 + 3 + 2 + 9 = 5 + 9 + 2 + 7 。
</pre>

#### 提示:
* <code>2 <= num.length <= 10<sup>5</sup></code>
* `num.length` 是 **偶数** 。
* `num` 只包含数字字符和 `'?'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sum_game(num: String) -> bool {
        let num = num.as_bytes();
        let mut diff = 0;
        let mut diff_mask = 0;

        for i in 0..num.len() / 2 {
            if num[i] != b'?' {
                diff += (num[i] - b'0') as i32;
            } else {
                diff_mask -= 1;
            }
        }
        for i in num.len() / 2..num.len() {
            if num[i] != b'?' {
                diff -= (num[i] - b'0') as i32;
            } else {
                diff_mask += 1;
            }
        }

        diff_mask % 2 != 0 || diff_mask / 2 * 9 != diff
    }
}
```
