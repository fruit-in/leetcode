# 1654. Minimum Jumps to Reach Home
A certain bug's home is on the x-axis at position `x`. Help them get there from position `0`.

The bug jumps according to the following rules:

* It can jump exactly `a` positions **forward** (to the right).
* It can jump exactly `b` positions **backward** (to the left).
* It cannot jump backward twice in a row.
* It cannot jump to any `forbidden` positions.

The bug may jump forward **beyond** its home, but it **cannot jump** to positions numbered with **negative** integers.

Given an array of integers `forbidden`, where `forbidden[i]` means that the bug cannot jump to the position `forbidden[i]`, and integers `a`, `b`, and `x`, return *the minimum number of jumps needed for the bug to reach its home*. If there is no possible sequence of jumps that lands the bug on position `x`, return `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> forbidden = [14,4,18,1,15], a = 3, b = 15, x = 9
<strong>Output:</strong> 3
<strong>Explanation:</strong> 3 jumps forward (0 -> 3 -> 6 -> 9) will get the bug home.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> forbidden = [8,3,16,6,12,20], a = 15, b = 13, x = 11
<strong>Output:</strong> -1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> forbidden = [1,6,2,14,5,17,4], a = 16, b = 9, x = 7
<strong>Output:</strong> 2
<strong>Explanation:</strong> One jump forward (0 -> 16) then one jump backward (16 -> 7) will get the bug home.
</pre>

#### Constraints:
* `1 <= forbidden.length <= 1000`
* `1 <= a, b, forbidden[i] <= 2000`
* `0 <= x <= 2000`
* All the elements in `forbidden` are distinct.
* Position `x` is not forbidden.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        let forbidden = forbidden.into_iter().collect::<HashSet<_>>();
        let mut queue = VecDeque::from([(0, 0, true)]);
        let mut visited = HashSet::from([(0, true), (0, false)]);

        while let Some((pos, jumps, isforward)) = queue.pop_front() {
            if pos == x {
                return jumps;
            }

            if pos + a <= 10000
                && !forbidden.contains(&(pos + a))
                && visited.insert((pos + a, true))
            {
                queue.push_back((pos + a, jumps + 1, true));
            }
            if isforward
                && pos - b >= 0
                && !forbidden.contains(&(pos - b))
                && visited.insert((pos - b, false))
            {
                queue.push_back((pos - b, jumps + 1, false));
            }
        }

        -1
    }
}
```
