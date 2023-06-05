# 2038. 如果相邻两个颜色均相同则删除当前颜色
总共有 `n` 个颜色片段排成一列，每个颜色片段要么是 `'A'` 要么是 `'B'` 。给你一个长度为 `n` 的字符串 `colors` ，其中 `colors[i]` 表示第 `i` 个颜色片段的颜色。

Alice 和 Bob 在玩一个游戏，他们 **轮流** 从这个字符串中删除颜色。Alice **先手** 。

* 如果一个颜色片段为 `'A'` 且 **相邻两个颜色** 都是颜色 `'A'` ，那么 Alice 可以删除该颜色片段。Alice **不可以** 删除任何颜色 `'B'` 片段。
* 如果一个颜色片段为 `'B'` 且 **相邻两个颜色** 都是颜色 `'B'` ，那么 Bob 可以删除该颜色片段。Bob **不可以** 删除任何颜色 `'A'` 片段。
* Alice 和 Bob **不能** 从字符串两端删除颜色片段。
* 如果其中一人无法继续操作，则该玩家 **输** 掉游戏且另一玩家 **获胜** 。

假设 Alice 和 Bob 都采用最优策略，如果 Alice 获胜，请返回 `true`，否则 Bob 获胜，返回 `false`。

#### 示例 1:
<pre>
<strong>输入:</strong> colors = "AAABABB"
<strong>输出:</strong> true
<strong>解释:</strong>
AAABABB -> AABABB
Alice 先操作。
她删除从左数第二个 'A' ，这也是唯一一个相邻颜色片段都是 'A' 的 'A' 。

现在轮到 Bob 操作。
Bob 无法执行任何操作，因为没有相邻位置都是 'B' 的颜色片段 'B' 。
因此，Alice 获胜，返回 true 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> colors = "AA"
<strong>输出:</strong> false
<strong>解释:</strong>
Alice 先操作。
只有 2 个 'A' 且它们都在字符串的两端，所以她无法执行任何操作。
因此，Bob 获胜，返回 false 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> colors = "ABBBBBBBAAA"
<strong>输出:</strong> false
<strong>解释:</strong>
ABBBBBBBAAA -> ABBBBBBBAA
Alice 先操作。
她唯一的选择是删除从右数起第二个 'A' 。

ABBBBBBBAA -> ABBBBBBAA
接下来轮到 Bob 操作。
他有许多选择，他可以选择任何一个 'B' 删除。

然后轮到 Alice 操作，她无法删除任何片段。
所以 Bob 获胜，返回 false 。
</pre>

#### 提示:
* <code>1 <= colors.length <= 10<sup>5</sup></code>
* `colors` 只包含字母 `'A'` 和 `'B'`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let colors = colors.as_bytes();
        let mut count = 1;
        let mut count_a = 0;
        let mut count_b = 0;

        for i in 1..colors.len() + 1 {
            if i >= colors.len() || colors[i] != colors[i - 1] {
                if colors[i - 1] == b'A' {
                    count_a += (count - 2).max(0);
                } else {
                    count_b += (count - 2).max(0);
                }

                count = 0;
            }

            count += 1;
        }

        count_a > count_b
    }
}
```
