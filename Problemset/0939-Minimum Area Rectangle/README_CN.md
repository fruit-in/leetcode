# 939. 最小面积矩形
给定在 xy 平面上的一组点，确定由这些点组成的矩形的最小面积，其中矩形的边平行于 x 轴和 y 轴。

如果没有任何矩形，就返回 0。

#### 示例 1:
<pre>
<strong>输入:</strong> [[1,1],[1,3],[3,1],[3,3],[2,2]]
<strong>输出:</strong> 4
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [[1,1],[1,3],[3,1],[3,3],[4,1],[4,3]]
<strong>输出:</strong> 2
</pre>

#### 提示:
1. `1 <= points.length <= 500`
2. `0 <= points[i][0] <= 40000`
3. `0 <= points[i][1] <= 40000`
4. 所有的点都是不同的。

## 题解 (Python)

### 1. 排序
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

## 题解 (Rust)

### 1. 排序
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
