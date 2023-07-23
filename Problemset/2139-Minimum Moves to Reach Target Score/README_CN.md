# 2139. 得到目标值的最少行动次数
你正在玩一个整数游戏。从整数 `1` 开始，期望得到整数 `target` 。

在一次行动中，你可以做下述两种操作之一：
* **递增**，将当前整数的值加 1（即， `x = x + 1`）。
* **加倍**，使当前整数的值翻倍（即，`x = 2 * x`）。

在整个游戏过程中，你可以使用 **递增** 操作 **任意** 次数。但是只能使用 **加倍** 操作 **至多** `maxDoubles` 次。

给你两个整数 `target` 和 `maxDoubles` ，返回从 1 开始得到 `target` 需要的最少行动次数。

#### 示例 1:
<pre>
<strong>输入:</strong> target = 5, maxDoubles = 0
<strong>输出:</strong> 4
<strong>解释:</strong> 一直递增 1 直到得到 target 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> target = 19, maxDoubles = 2
<strong>输出:</strong> 7
<strong>解释:</strong> 最初，x = 1 。
递增 3 次，x = 4 。
加倍 1 次，x = 8 。
递增 1 次，x = 9 。
加倍 1 次，x = 18 。
递增 1 次，x = 19 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> target = 10, maxDoubles = 4
<strong>输出:</strong> 4
<strong>解释:</strong> 最初，x = 1 。
递增 1 次，x = 2 。
加倍 1 次，x = 4 。
递增 1 次，x = 5 。
加倍 1 次，x = 10 。
</pre>

#### 提示:
* <code>1 <= target <= 10<sup>9</sup></code>
* `0 <= maxDoubles <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_moves(target: i32, max_doubles: i32) -> i32 {
        let mut target = target;
        let mut max_doubles = max_doubles;
        let mut ret = 0;

        while target > 1 {
            if max_doubles == 0 {
                return ret + target - 1;
            } else if target % 2 == 1 {
                target -= 1;
            } else {
                target /= 2;
                max_doubles -= 1;
            }

            ret += 1;
        }

        ret
    }
}
```
