# 2571. 将整数减少到零需要的最少操作数
给你一个正整数 `n` ，你可以执行下述操作 **任意** 次：
* `n` 加上或减去 `2` 的某个 **幂**

返回使 `n` 等于 `0` 需要执行的 **最少** 操作数。

如果 <code>x == 2<sup>i</sup></code> 且其中 `i >= 0` ，则数字 `x` 是 `2` 的幂。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 39
<strong>输出:</strong> 3
<strong>解释:</strong> 我们可以执行下述操作：
- n 加上 2<sup>0</sup> = 1 ，得到 n = 40 。
- n 减去 2<sup>3</sup> = 8 ，得到 n = 32 。
- n 减去 2<sup>5</sup> = 32 ，得到 n = 0 。
可以证明使 n 等于 0 需要执行的最少操作数是 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 54
<strong>输出:</strong> 3
<strong>解释:</strong> 我们可以执行下述操作：
- n 加上 2<sup>1</sup> = 2 ，得到 n = 56 。
- n 加上 2<sup>3</sup> = 8 ，得到 n = 64 。
- n 减去 2<sup>6</sup> = 64 ，得到 n = 0 。
使 n 等于 0 需要执行的最少操作数是 3 。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let m = n.ilog2() + 1;
        let mut deque = VecDeque::from([(n as usize, 0)]);
        let mut visited = vec![false; (1 << m) + 1];
        visited[n as usize] = true;

        while let Some((x, steps)) = deque.pop_front() {
            if x == 0 {
                return steps;
            }

            for i in 0..=m {
                if x + (1 << i) < visited.len() && !visited[x + (1 << i)] {
                    deque.push_back((x + (1 << i), steps + 1));
                    visited[x + (1 << i)] = true;
                }
                if x >= 1 << i && !visited[x - (1 << i)] {
                    deque.push_back((x - (1 << i), steps + 1));
                    visited[x - (1 << i)] = true;
                }
            }
        }

        unreachable!()
    }
}
```
