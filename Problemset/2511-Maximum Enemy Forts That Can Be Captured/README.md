# 2511. Maximum Enemy Forts That Can Be Captured
You are given a **0-indexed** integer array `forts` of length `n` representing the positions of several forts. `forts[i]` can be `-1`, `0`, or `1` where:

* `-1` represents there is **no fort** at the <code>i<sup>th</sup></code> position.
* `0` indicates there is an **enemy** fort at the <code>i<sup>th</sup></code> position.
* `1` indicates the fort at the <code>i<sup>th</sup></code> the position is under your command.

Now you have decided to move your army from one of your forts at position `i` to an empty position `j` such that:

* `0 <= i, j <= n - 1`
* The army travels over enemy forts **only**. Formally, for all `k` where `min(i,j) < k < max(i,j)`, `forts[k] == 0`.

While moving the army, all the enemy forts that come in the way are **captured**.

Return *the **maximum** number of enemy forts that can be captured*. In case it is **impossible** to move your army, or you do not have any fort under your command, return `0`.


#### Example 1:
<pre>
<strong>Input:</strong> forts = [1,0,0,-1,0,0,0,0,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong>
- Moving the army from position 0 to position 3 captures 2 enemy forts, at 1 and 2.
- Moving the army from position 8 to position 3 captures 4 enemy forts.
Since 4 is the maximum number of enemy forts that can be captured, we return 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> forts = [0,0,1,-1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Since no enemy fort can be captured, 0 is returned.
</pre>

#### Constraints:
* `1 <= forts.length <= 1000`
* `-1 <= forts[i] <= 1`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let mut prev = (0, 0);
        let mut ret = 0;

        for i in 0..forts.len() {
            if forts[i] != 0 {
                if prev.0 != 0 && prev.0 != forts[i] {
                    ret = ret.max(i - prev.1 - 1);
                }

                prev = (forts[i], i);
            }
        }

        ret as i32
    }
}
```
