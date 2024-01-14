# 403. Frog Jump
A frog is crossing a river. The river is divided into some number of units, and at each unit, there may or may not exist a stone. The frog can jump on a stone, but it must not jump into the water.

Given a list of `stones` positions (in units) in sorted **ascending order**, determine if the frog can cross the river by landing on the last stone. Initially, the frog is on the first stone and assumes the first jump must be `1` unit.

If the frog's last jump was `k` units, its next jump must be either `k - 1`, `k`, or `k + 1` units. The frog can only jump in the forward direction.

#### Example 1:
<pre>
<strong>Input:</strong> stones = [0,1,3,5,6,8,12,17]
<strong>Output:</strong> true
<strong>Explanation:</strong> The frog can jump to the last stone by jumping 1 unit to the 2nd stone, then 2 units to the 3rd stone, then 2 units to the 4th stone, then 3 units to the 6th stone, 4 units to the 7th stone, and 5 units to the 8th stone.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> stones = [0,1,2,3,4,8,9,11]
<strong>Output:</strong> false
<strong>Explanation:</strong> There is no way to jump to the last stone as the gap between the 5th and 6th stone is too large.
</pre>

#### Constraints:
* `2 <= stones.length <= 2000`
* <code>0 <= stones[i] <= 2<sup>31</sup> - 1</code>
* `stones[0] == 0`
* `stones` is sorted in a strictly increasing order.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        if stones[1] != 1 {
            return false;
        }

        let stones = stones.into_iter().map(|s| s as usize).collect::<Vec<_>>();
        let mut indices = stones
            .iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect::<HashMap<_, _>>();
        let mut dp = vec![vec![false; stones.len()]; stones.len()];
        dp[1][1] = true;

        for i in 1..stones.len() {
            for k in 1..=i {
                if dp[i][k] {
                    if i == stones.len() - 1 {
                        return true;
                    }

                    if let Some(&j) = indices.get(&(stones[i] + k - 1)) {
                        dp[j][k - 1] = true;
                    }
                    if let Some(&j) = indices.get(&(stones[i] + k)) {
                        dp[j][k] = true;
                    }
                    if let Some(&j) = indices.get(&(stones[i] + k + 1)) {
                        dp[j][k + 1] = true;
                    }
                }
            }
        }

        false
    }
}
```
