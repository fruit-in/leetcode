# 2147. 分隔长廊的方案数
在一个图书馆的长廊里，有一些座位和装饰植物排成一列。给你一个下标从 **0** 开始，长度为 `n` 的字符串 `corridor` ，它包含字母 `'S'` 和 `'P'` ，其中每个 `'S'` 表示一个座位，每个 `'P'` 表示一株植物。

在下标 `0` 的左边和下标 `n - 1` 的右边 **已经** 分别各放了一个屏风。你还需要额外放置一些屏风。每一个位置 `i - 1` 和 `i` 之间（`1 <= i <= n - 1`），至多能放一个屏风。

请你将走廊用屏风划分为若干段，且每一段内都 **恰好有两个座位** ，而每一段内植物的数目没有要求。可能有多种划分方案，如果两个方案中有任何一个屏风的位置不同，那么它们被视为 **不同** 方案。

请你返回划分走廊的方案数。由于答案可能很大，请你返回它对 <code>10<sup>9</sup> + 7</code> 取余 的结果。如果没有任何方案，请返回 `0` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/12/04/1.png)
<pre>
<strong>输入:</strong> corridor = "SSPPSPS"
<strong>输出:</strong> 3
<strong>解释:</strong> 总共有 3 种不同分隔走廊的方案。
上图中黑色的竖线表示已经放置好的屏风。
上图每种方案中，每一段都恰好有 两个 座位。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/12/04/2.png)
<pre>
<strong>输入:</strong> corridor = "PPSPSP"
<strong>输出:</strong> 1
<strong>解释:</strong> 只有 1 种分隔走廊的方案，就是不放置任何屏风。
放置任何的屏风都会导致有一段无法恰好有 2 个座位。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/12/12/3.png)
<pre>
<strong>输入:</strong> corridor = "S"
<strong>输出:</strong> 0
<strong>解释:</strong> 没有任何方案，因为总是有一段无法恰好有 2 个座位。
</pre>

#### 提示:
* `n == corridor.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `corridor[i]` 要么是 `'S'` ，要么是 `'P'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut count_s = 0;
        let mut count_p = vec![];

        for c in corridor.chars() {
            if c == 'S' {
                count_s += 1;
                if count_s % 2 == 0 {
                    count_p.push(0_i64);
                }
            } else if count_s > 0 && count_s % 2 == 0 {
                *count_p.last_mut().unwrap() += 1;
            }
        }

        if count_s == 0 || count_s % 2 == 1 {
            return 0;
        }

        count_p.pop();

        count_p
            .iter()
            .fold(1, |acc, x| acc * (x + 1) % 1_000_000_007) as i32
    }
}
```
