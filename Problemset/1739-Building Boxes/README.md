# 1739. Building Boxes
You have a cubic storeroom where the width, length, and height of the room are all equal to `n` units. You are asked to place `n` boxes in this room where each box is a cube of unit side length. There are however some rules to placing the boxes:
* You can place the boxes anywhere on the floor.
* If box `x` is placed on top of the box `y`, then each side of the four vertical sides of the box `y` **must** either be adjacent to another box or to a wall.

Given an integer `n`, return *the **minimum** possible number of boxes touching the floor*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/01/04/3-boxes.png)
<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> The figure above is for the placement of the three boxes.
These boxes are placed in the corner of the room, where the corner is on the left side.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/01/04/4-boxes.png)
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> 3
<strong>Explanation:</strong> The figure above is for the placement of the four boxes.
These boxes are placed in the corner of the room, where the corner is on the left side.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/01/04/10-boxes.png)
<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> 6
<strong>Explanation:</strong> The figure above is for the placement of the ten boxes.
These boxes are placed in the corner of the room, where the corner is on the back side.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_boxes(n: i32) -> i32 {
        let n = n as i64;
        let mut lo = 1;
        let mut hi = n;

        while lo < hi {
            let mid = (lo + hi) / 2;
            let mut boxes = 0;
            let mut x = (mid as f64 * 2.).sqrt() as i64;
            let mut y = mid - x * (x + 1) / 2;

            if y < 0 {
                y += x;
                x -= 1;
            }

            while x > 0 {
                boxes += x * (x + 1) / 2 + y;
                x -= 1;
                y -= 1;
                y = y.max(0);
            }

            if boxes < n {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        hi as i32
    }
}
```
