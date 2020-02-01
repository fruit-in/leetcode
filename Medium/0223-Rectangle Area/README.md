# 223. Rectangle Area
Find the total area covered by two **rectilinear** rectangles in a **2D** plane.

Each rectangle is defined by its bottom left corner and top right corner as shown in the figure.

![](https://assets.leetcode.com/uploads/2018/10/22/rectangle_area.png)

#### Example:
<pre>
<strong>Input:</strong> A = -3, B = 0, C = 3, D = 4, E = 0, F = -1, G = 9, H = 2
<strong>Output:</strong> 45
</pre>

#### Note:
Assume that the total area is never beyond the maximum possible value of **int**.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        let area0 = (c - a) * (d - b);
        let area1 = (g - e) * (h - f);
        let area2 = (c.min(g).saturating_sub(a.max(e))).max(0) *
            (d.min(h).saturating_sub(b.max(f))).max(0);

        area0 + area1 - area2
    }
}
```
