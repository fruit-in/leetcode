# 397. Integer Replacement
Given a positive integer `n`, you can apply one of the following operations:
1. If `n` is even, replace `n` with `n / 2`.
2. If `n` is odd, replace `n` with either `n + 1` or `n - 1`.

Return *the minimum number of operations needed for `n` to become `1`*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 8
<strong>Output:</strong> 3
<strong>Explanation:</strong> 8 -> 4 -> 2 -> 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 7
<strong>Output:</strong> 4
<strong>Explanation:</strong> 7 -> 8 -> 4 -> 2 -> 1
or 7 -> 6 -> 3 -> 2 -> 1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> 2
</pre>

#### Constraints:
* <code>1 <= n <= 2<sup>31</sup> - 1</code>

## Solutions (Rust)

### 1. BFS
```Rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut nums = vec![(n as i64, 0)].into_iter().collect::<VecDeque<_>>();
        let mut seen = vec![n as i64].into_iter().collect::<HashSet<_>>();

        while let Some((x, y)) = nums.pop_front() {
            if x == 1 {
                return y;
            }
            if x % 2 == 0 && !seen.contains(&(x / 2)) {
                nums.push_back((x / 2, y + 1));
                seen.insert(x / 2);
            } else if x % 2 == 1 {
                if !seen.contains(&(x + 1)) {
                    nums.push_back((x + 1, y + 1));
                    seen.insert(x + 1);
                }
                if !seen.contains(&(x - 1)) {
                    nums.push_back((x - 1, y + 1));
                    seen.insert(x - 1);
                }
            }
        }

        0
    }
}
```
