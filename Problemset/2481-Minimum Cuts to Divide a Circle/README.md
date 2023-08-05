# 2481. Minimum Cuts to Divide a Circle
A **valid cut** in a circle can be:

* A cut that is represented by a straight line that touches two points on the edge of the circle and passes through its center, or
* A cut that is represented by a straight line that touches one point on the edge of the circle and its center.

Some valid and invalid cuts are shown in the figures below.

![](https://assets.leetcode.com/uploads/2022/10/29/alldrawio.png)

Given the integer `n`, return *the **minimum** number of cuts needed to divide a circle into* `n` *equal slices*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/10/24/11drawio.png)
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong>
The above figure shows how cutting the circle twice through the middle divides it into 4 equal slices.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/10/24/22drawio.png)
<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong>
At least 3 cuts are needed to divide the circle into 3 equal slices.
It can be shown that less than 3 cuts cannot result in 3 slices of equal size and shape.
Also note that the first cut will not divide the circle into distinct parts.
</pre>

#### Constraints:
* `1 <= n <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 || n % 2 == 0 {
            n / 2
        } else {
            n
        }
    }
}
```
