# 2249. Count Lattice Points Inside a Circle
Given a 2D integer array `circles` where <code>circles[i] = [x<sub>i</sub>, y<sub>i</sub>, r<sub>i</sub>]</code> represents the center <code>(x<sub>i</sub>, y<sub>i</sub>)</code> and radius <code>r<sub>i</sub></code> of the <code>i<sup>th</sup></code> circle drawn on a grid, return *the **number of lattice points** that are present inside **at least one** circle*.

**Note:**

* A **lattice point** is a point with integer coordinates.
* Points that lie **on the circumference of a circle** are also considered to be inside it.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/03/02/exa-11.png)
<pre>
<strong>Input:</strong> circles = [[2,2,1]]
<strong>Output:</strong> 5
<strong>Explanation:</strong>
The figure above shows the given circle.
The lattice points present inside the circle are (1, 2), (2, 1), (2, 2), (2, 3), and (3, 2) and are shown in green.
Other points such as (1, 1) and (1, 3), which are shown in red, are not considered inside the circle.
Hence, the number of lattice points present inside at least one circle is 5.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/03/02/exa-22.png)
<pre>
<strong>Input:</strong> circles = [[2,2,2],[3,4,1]]
<strong>Output:</strong> 16
<strong>Explanation:</strong>
The figure above shows the given circles.
There are exactly 16 lattice points which are present inside at least one circle.
Some of them are (0, 2), (2, 0), (2, 4), (3, 2), and (4, 4).
</pre>

#### Constraints:
* `1 <= circles.length <= 200`
* `circles[i].length == 3`
* <code>1 <= x<sub>i</sub>, y<sub>i</sub> <= 100</code>
* <code>1 <= r<sub>i</sub> <= min(x<sub>i</sub>, y<sub>i</sub>)</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_lattice_points(circles: Vec<Vec<i32>>) -> i32 {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        let mut ret = 0;

        for circle in &circles {
            min_x = min_x.min(circle[0] - circle[2]);
            min_y = min_y.min(circle[1] - circle[2]);
            max_x = max_x.max(circle[0] + circle[2]);
            max_y = max_y.max(circle[1] + circle[2]);
        }

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for circle in &circles {
                    if (circle[0] - x).pow(2) + (circle[1] - y).pow(2) <= circle[2].pow(2) {
                        ret += 1;
                        break;
                    }
                }
            }
        }

        ret
    }
}
```
