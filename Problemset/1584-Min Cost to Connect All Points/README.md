# 1584. Min Cost to Connect All Points
You are given an array `points` representing integer coordinates of some points on a 2D-plane, where <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code>.

The cost of connecting two points <code>[x<sub>i</sub>, y<sub>i</sub>]</code> and <code>[x<sub>j</sub>, y<sub>j</sub>]</code> is the **manhattan distance** between them: <code>|x<sub>i</sub> - x<sub>j</sub>| + |y<sub>i</sub> - y<sub>j</sub>|</code>, where `|val|` denotes the absolute value of `val`.

Return *the minimum cost to make all points connected*. All points are connected if there is **exactly one** simple path between any two points.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/08/26/d.png)
<pre>
<strong>Input:</strong> points = [[0,0],[2,2],[3,10],[5,2],[7,0]]
<strong>Output:</strong> 20
<strong>Explanation:</strong>
<img src="https://assets.leetcode.com/uploads/2020/08/26/c.png">
We can connect the points as shown above to get the minimum cost of 20.
Notice that there is a unique path between every pair of points.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> points = [[3,12],[-2,5],[-4,1]]
<strong>Output:</strong> 18
</pre>

#### Constraints:
* `1 <= points.length <= 1000`
* <code>-10<sup>6</sup> <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>6</sup></code>
* All pairs <code>(x<sub>i</sub>, y<sub>i</sub>)</code> are distinct.

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut used = vec![false; points.len()];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
        let mut ret = 0;

        for _ in 0..points.len() {
            while let Some((Reverse(dist), i)) = heap.pop() {
                if !used[i] {
                    let (xi, yi) = (points[i][0], points[i][1]);
                    used[i] = true;
                    ret += dist;
                    for j in 0..points.len() {
                        if !used[j] {
                            let (xj, yj) = (points[j][0], points[j][1]);
                            let dist = (xi - xj).abs() + (yi - yj).abs();
                            heap.push((Reverse(dist), j));
                        }
                    }
                    break;
                }
            }
        }

        ret
    }
}
```
