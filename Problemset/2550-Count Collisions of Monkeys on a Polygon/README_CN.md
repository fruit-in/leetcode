# 2550. 猴子碰撞的方法数
现在有一个正凸多边形，其上共有 `n` 个顶点。顶点按顺时针方向从 `0` 到 `n - 1` 依次编号。每个顶点上 **正好有一只猴子** 。下图中是一个 6 个顶点的凸多边形。

![](https://assets.leetcode.com/uploads/2023/01/22/hexagon.jpg)

每个猴子同时移动到相邻的顶点。顶点 `i` 的相邻顶点可以是：

* 顺时针方向的顶点 `(i + 1) % n` ，或
* 逆时针方向的顶点 `(i - 1 + n) % n` 。

如果移动后至少有两只猴子停留在同一个顶点上或者相交在一条边上，则会发生 **碰撞** 。

返回猴子至少发生 **一次碰撞** 的移动方法数。由于答案可能非常大，请返回对 <code>10<sup>9</sup>+7</code> 取余后的结果。

**注意**，每只猴子只能移动一次。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> 6
<strong>解释:</strong> 共计 8 种移动方式。
下面列出两种会发生碰撞的方式：
- 猴子 1 顺时针移动；猴子 2 逆时针移动；猴子 3 顺时针移动。猴子 1 和猴子 2 碰撞。
- 猴子 1 逆时针移动；猴子 2 逆时针移动；猴子 3 顺时针移动。猴子 1 和猴子 3 碰撞。
可以证明，有 6 种让猴子碰撞的方法。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 4
<strong>输出:</strong> 14
<strong>解释:</strong> 可以证明，有 14 种让猴子碰撞的方法。
</pre>

#### 提示:
* <code>3 <= n <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn monkey_move(n: i32) -> i32 {
        (Self::pow2(n) - 2).rem_euclid(1_000_000_007) as i32
    }

    fn pow2(n: i32) -> i64 {
        if n == 0 {
            1
        } else if n % 2 == 0 {
            Self::pow2(n / 2).pow(2) % 1_000_000_007
        } else {
            (Self::pow2(n - 1) * 2) % 1_000_000_007
        }
    }
}
```