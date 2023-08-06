# 2485. Find the Pivot Integer
Given a positive integer `n`, find the **pivot integer** `x` such that:

* The sum of all elements between `1` and `x` inclusively equals the sum of all elements between `x` and `n` inclusively.

Return *the pivot integer* `x`. If no such integer exists, return `-1`. It is guaranteed that there will be at most one pivot index for the given input.

#### Example 1:
<pre>
<strong>Input:</strong> n = 8
<strong>Output:</strong> 6
<strong>Explanation:</strong> 6 is the pivot integer since: 1 + 2 + 3 + 4 + 5 + 6 = 6 + 7 + 8 = 21.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> 1 is the pivot integer since: 1 = 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> -1
<strong>Explanation:</strong> It can be proved that no such integer exist.
</pre>

#### Constraints:
* `1 <= n <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let x = ((n * (n + 1) / 2) as f64).sqrt() as i32;

        if x * x * 2 == n * (n + 1) {
            x
        } else {
            -1
        }
    }
}
```
