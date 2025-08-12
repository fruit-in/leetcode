# 2571. Minimum Operations to Reduce an Integer to 0
You are given a positive integer `n`, you can do the following operation **any** number of times:
* Add or subtract a **power** of `2` from `n`.

Return *the **minimum** number of operations to make* `n` *equal to* `0`.

A number `x` is power of `2` if <code>x == 2<sup>i</sup></code> where `i >= 0`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 39
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can do the following operations:
- Add 2<sup>0</sup> = 1 to n, so now n = 40.
- Subtract 2<sup>3</sup> = 8 from n, so now n = 32.
- Subtract 2<sup>5</sup> = 32 from n, so now n = 0.
It can be shown that 3 is the minimum number of operations we need to make n equal to 0.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 54
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can do the following operations:
- Add 2<sup>1</sup> = 2 to n, so now n = 56.
- Add 2<sup>3</sup> = 8 to n, so now n = 64.
- Subtract 2<sup>6</sup> = 64 from n, so now n = 0.
So the minimum number of operations is 3.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
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
