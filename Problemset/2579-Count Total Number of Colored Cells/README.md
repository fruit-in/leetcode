# 2579. Count Total Number of Colored Cells
There exists an infinitely large two-dimensional grid of uncolored unit cells. You are given a positive integer `n`, indicating that you must do the following routine for `n` minutes:

* At the first minute, color **any** arbitrary unit cell blue.
* Every minute thereafter, color blue **every** uncolored cell that touches a blue cell.

Below is a pictorial representation of the state of the grid after minutes 1, 2, and 3.

![](https://assets.leetcode.com/uploads/2023/01/10/example-copy-2.png)

Return *the number of **colored cells** at the end of* `n` *minutes*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> After 1 minute, there is only 1 blue cell, so we return 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 5
<strong>Explanation:</strong> After 2 minutes, there are 4 colored cells on the boundary and 1 in the center, so we return 5.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        (1..n as i64).fold(1, |acc, x| acc + x * 4)
    }
}
```
