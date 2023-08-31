# 1620. Coordinate With Maximum Network Quality
You are given an array of network towers `towers`, where <code>towers[i] = [x<sub>i</sub>, y<sub>i</sub>, q<sub>i</sub>]</code> denotes the <code>i<sup>th</sup></code> network tower with location <code>(x<sub>i</sub>, y<sub>i</sub>)</code> and quality factor <code>q<sub>i</sub></code>. All the coordinates are **integral coordinates** on the X-Y plane, and the distance between the two coordinates is the **Euclidean distance**.

You are also given an integer `radius` where a tower is **reachable** if the distance is **less than or equal to** `radius`. Outside that distance, the signal becomes garbled, and the tower is **not reachable**.

The signal quality of the <code>i<sup>th</sup></code> tower at a coordinate `(x, y)` is calculated with the formula <code>⌊q<sub>i</sub> / (1 + d)⌋</code>, where `d` is the distance between the tower and the coordinate. The **network quality** at a coordinate is the sum of the signal qualities from all the **reachable** towers.

Return *the array* <code>[c<sub>x</sub>, c<sub>y</sub>]</code> *representing the **integral** coordinate* <code>(c<sub>x</sub>, c<sub>y</sub>)</code> *where the **network quality** is maximum. If there are multiple coordinates with the same **network quality**, return the lexicographically minimum **non-negative** coordinate*.

**Note:**

* A coordinate `(x1, y1)` is lexicographically smaller than `(x2, y2)` if either:
    * `x1 < x2`, or
    * `x1 == x2` and `y1 < y2`.
* `⌊val⌋` is the greatest integer less than or equal to `val` (the floor function).

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/22/untitled-diagram.png)
<pre>
<strong>Input:</strong> towers = [[1,2,5],[2,1,7],[3,1,9]], radius = 2
<strong>Output:</strong> [2,1]
<strong>Explanation:</strong> At coordinate (2, 1) the total quality is 13.
- Quality of 7 from (2, 1) results in ⌊7 / (1 + sqrt(0)⌋ = ⌊7⌋ = 7
- Quality of 5 from (1, 2) results in ⌊5 / (1 + sqrt(2)⌋ = ⌊2.07⌋ = 2
- Quality of 9 from (3, 1) results in ⌊9 / (1 + sqrt(1)⌋ = ⌊4.5⌋ = 4
No other coordinate has a higher network quality.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> towers = [[23,11,21]], radius = 9
<strong>Output:</strong> [23,11]
<strong>Explanation:</strong> Since there is only one tower, the network quality is highest right at the tower's location.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> towers = [[1,2,13],[2,1,7],[0,1,9]], radius = 2
<strong>Output:</strong> [1,2]
<strong>Explanation:</strong> Coordinate (1, 2) has the highest network quality.
</pre>

#### Constraints:
* `1 <= towers.length <= 50`
* `towers[i].length == 3`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub>, q<sub>i</sub> <= 50</code>
* `1 <= radius <= 50`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let min_x = towers.iter().map(|t| t[0]).min().unwrap();
        let min_y = towers.iter().map(|t| t[1]).min().unwrap();
        let max_x = towers.iter().map(|t| t[0]).max().unwrap();
        let max_y = towers.iter().map(|t| t[1]).max().unwrap();
        let mut max_q = 0;
        let mut ret = vec![0, 0];

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let mut q = 0;

                for t in &towers {
                    let d = (((t[0] - x).pow(2) + (t[1] - y).pow(2)) as f64).sqrt();

                    if d <= radius as f64 {
                        q += (t[2] as f64 / (1.0 + d)) as i32;
                    }
                }

                if q > max_q {
                    max_q = q;
                    ret = vec![x, y];
                }
            }
        }

        ret
    }
}
```
