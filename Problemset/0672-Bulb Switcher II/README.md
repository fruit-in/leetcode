# 672. Bulb Switcher II
There is a room with `n` bulbs labeled from `1` to `n` that all are turned on initially, and **four buttons** on the wall. Each of the four buttons has a different functionality where:

* **Button 1:** Flips the status of all the bulbs.
* **Button 2:** Flips the status of all the bulbs with even labels (i.e., `2, 4, ...`).
* **Button 3:** Flips the status of all the bulbs with odd labels (i.e., `1, 3, ...`).
* **Button 4:** Flips the status of all the bulbs with a label `j = 3k + 1` where `k = 0, 1, 2, ...` (i.e., `1, 4, 7, 10, ...`).

You must make **exactly** `presses` button presses in total. For each press, you may pick **any** of the four buttons to press.

Given the two integers `n` and `presses`, return *the number of **different possible statuses** after performing all* `presses` *button presses*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 1, presses = 1
<strong>Output:</strong> 2
<strong>Explanation:</strong> Status can be:
- [off] by pressing button 1
- [on] by pressing button 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2, presses = 1
<strong>Output:</strong> 3
<strong>Explanation:</strong> Status can be:
- [off, off] by pressing button 1
- [on, off] by pressing button 2
- [off, on] by pressing button 3
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 3, presses = 1
<strong>Output:</strong> 4
<strong>Explanation:</strong> Status can be:
- [off, off, off] by pressing button 1
- [off, on, off] by pressing button 2
- [on, off, on] by pressing button 3
- [off, on, on] by pressing button 4
</pre>

#### Constraints:
* `1 <= n <= 1000`
* `0 <= presses <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        match (n, presses) {
            (_, 0) => 1,
            (1, _) => 2,
            (2, 1) => 3,
            (2, _) => 4,
            (_, 1) => 4,
            (_, 2) => 7,
            _ => 8,
        }
    }
}
```
