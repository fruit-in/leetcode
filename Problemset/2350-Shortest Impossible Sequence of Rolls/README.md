# 2350. Shortest Impossible Sequence of Rolls
You are given an integer array `rolls` of length `n` and an integer `k`. You roll a `k` sided dice numbered from `1` to `k`, `n` times, where the result of the <code>i<sup>th</sup></code> roll is `rolls[i]`.

Return the length of the shortest sequence of rolls so that there's no such in `rolls`.

A sequence of rolls of length `len` is the result of rolling a `k` sided dice `len` times.

#### Example 1:
<pre>
<strong>Input:</strong> rolls = [4,2,1,2,3,3,2,4,1], k = 4
<strong>Output:</strong> 3
<strong>Explanation:</strong> Every sequence of rolls of length 1, [1], [2], [3], [4], can be taken from rolls.
Every sequence of rolls of length 2, [1, 1], [1, 2], ..., [4, 4], can be taken from rolls.
The sequence [1, 4, 2] cannot be taken from rolls, so we return 3.
Note that there are other sequences that cannot be taken from rolls.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> rolls = [1,1,2,2], k = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> Every sequence of rolls of length 1, [1], [2], can be taken from rolls.
The sequence [2, 1] cannot be taken from rolls, so we return 2.
Note that there are other sequences that cannot be taken from rolls but [2, 1] is the shortest.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> rolls = [1,1,3,2,2,2,3,3], k = 4
<strong>Output:</strong> 1
<strong>Explanation:</strong> The sequence [4] cannot be taken from rolls, so we return 1.
Note that there are other sequences that cannot be taken from rolls but [4] is the shortest.
</pre>

#### Constraints:
* `n == rolls.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= rolls[i] <= k <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        let mut set = HashSet::new();
        let mut ret = 1;

        for x in &rolls {
            set.insert(x);
            if set.len() as i32 == k {
                set = HashSet::new();
                ret += 1;
            }
        }

        ret
    }
}
```
