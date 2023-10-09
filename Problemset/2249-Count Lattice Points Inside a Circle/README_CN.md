# 2249. 统计圆内格点数目
给你一个二维整数数组 `circles` ，其中 <code>circles[i] = [x<sub>i</sub>, y<sub>i</sub>, r<sub>i</sub>]</code> 表示网格上圆心为 <code>(x<sub>i</sub>, y<sub>i</sub>)</code> 且半径为 <code>r<sub>i</sub></code> 的第 `i` 个圆，返回出现在 **至少一个** 圆内的 **格点数目** 。

**注意：**

* **格点** 是指整数坐标对应的点。
* **圆周上的点** 也被视为出现在圆内的点。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/03/02/exa-11.png)
<pre>
<strong>输入:</strong> circles = [[2,2,1]]
<strong>输出:</strong> 5
<strong>解释:</strong>
给定的圆如上图所示。
出现在圆内的格点为 (1, 2)、(2, 1)、(2, 2)、(2, 3) 和 (3, 2)，在图中用绿色标识。
像 (1, 1) 和 (1, 3) 这样用红色标识的点，并未出现在圆内。
因此，出现在至少一个圆内的格点数目是 5 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/03/02/exa-22.png)
<pre>
<strong>输入:</strong> circles = [[2,2,2],[3,4,1]]
<strong>输出:</strong> 16
<strong>解释:</strong>
给定的圆如上图所示。
共有 16 个格点出现在至少一个圆内。
其中部分点的坐标是 (0, 2)、(2, 0)、(2, 4)、(3, 2) 和 (4, 4) 。
</pre>

#### 提示:
* `1 <= circles.length <= 200`
* `circles[i].length == 3`
* <code>1 <= x<sub>i</sub>, y<sub>i</sub> <= 100</code>
* <code>1 <= r<sub>i</sub> <= min(x<sub>i</sub>, y<sub>i</sub>)</code>

## 题解 (Rust)

### 1. 题解
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
