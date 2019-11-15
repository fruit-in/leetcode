# 447. 回旋镖的数量
给定平面上 *n* 对不同的点，“回旋镖” 是由点表示的元组 ```(i, j, k)``` ，其中 ```i``` 和 ```j``` 之间的距离和 ```i``` 和 ```k``` 之间的距离相等（**需要考虑元组的顺序**）。

找到所有回旋镖的数量。你可以假设 *n* 最大为 **500**，所有点的坐标在闭区间 **[-10000, 10000]** 中。

#### 示例:
<pre>
<strong>输入:</strong>
[[0,0],[1,0],[2,0]]
<strong>输出:</strong>
2
<strong>解释:</strong>
两个回旋镖为 <strong>[[1,0],[0,0],[2,0]]</strong> 和 <strong>[[1,0],[2,0],[0,0]]</strong>
</pre>

## 题解 (Rust)

### 1. 哈希表
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
