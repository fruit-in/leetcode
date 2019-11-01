# 447. Number of Boomerangs
Given *n* points in the plane that are all pairwise distinct, a "boomerang" is a tuple of points ```(i, j, k)``` such that the distance between ```i``` and ```j``` equals the distance between ```i``` and ```k``` (**the order of the tuple matters**).

Find the number of boomerangs. You may assume that *n* will be at most **500** and coordinates of points are all in the range **[-10000, 10000]** (inclusive).

#### Example:
<pre>
<strong>Input:</strong>
[[0,0],[1,0],[2,0]]
<strong>Output:</strong>
2
<strong>Explanation:</strong>
The two boomerangs are <strong>[[1,0],[0,0],[2,0]]</strong> and <strong>[[1,0],[2,0],[0,0]]</strong>
</pre>

## Solutions (Rust)

### 1. HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut len_cnt = HashMap::new();
        let mut ret = 0;

        for i in 0..points.len() {
            for j in 0..points.len() {
                let dx = points[i][0] - points[j][0];
                let dy = points[i][1] - points[j][1];
                *len_cnt.entry(dx * dx + dy * dy).or_insert(0) += 1;
            }

            ret += len_cnt.values().map(|v| v * (v - 1)).sum::<i32>();
            len_cnt.clear();
        }

        ret
    }
}
```
