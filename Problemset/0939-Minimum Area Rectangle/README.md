# 939. Minimum Area Rectangle
Given a set of points in the xy-plane, determine the minimum area of a rectangle formed from these points, with sides parallel to the x and y axes.

If there isn't any rectangle, return 0.

#### Example 1:
<pre>
<strong>Input:</strong> [[1,1],[1,3],[3,1],[3,3],[2,2]]
<strong>Output:</strong> 4
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [[1,1],[1,3],[3,1],[3,3],[4,1],[4,3]]
<strong>Output:</strong> 2
</pre>

#### Note:
1. `1 <= points.length <= 500`
2. `0 <= points[i][0] <= 40000`
3. `0 <= points[i][1] <= 40000`
4. All points are distinct.

## Solutions (Python)

### 1. Sort
```Python
class Solution:
    def minAreaRect(self, points: List[List[int]]) -> int:
        points_set = set()
        min_area = None
        points.sort()

        for i in range(len(points)):
            x0, y0 = points[i]

            for j in range(i):
                x1, y1 = points[j]

                if (x0, y1) in points_set and (x1, y0) in points_set:
                    area = (x0 - x1) * abs(y0 - y1)
                    min_area = area if min_area is None \
                        else min(min_area, area)

            points_set.add((x0, y0))

        return 0 if min_area is None else min_area
```

## Solutions (Rust)

### 1. Sort
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn min_area_rect(mut points: Vec<Vec<i32>>) -> i32 {
        let mut set = HashSet::new();
        let mut min_area = None;
        points.sort_unstable();

        for i in 0..points.len() {
            let (x0, y0) = (points[i][0], points[i][1]);

            for j in 0..i {
                let (x1, y1) = (points[j][0], points[j][1]);

                if set.contains(&(x0, y1)) && set.contains(&(x1, y0)) {
                    let area = (x0 - x1) * (y0 - y1).abs();
                    min_area = Some(min_area.map_or(area, |a: i32| a.min(area)));
                }
            }

            set.insert((x0, y0));
        }

        min_area.unwrap_or(0)
    }
}
```
