# 1620. 网络信号最好的坐标
给你一个数组 `towers` 和一个整数 `radius` 。

数组  `towers`  中包含一些网络信号塔，其中 <code>towers[i] = [x<sub>i</sub>, y<sub>i</sub>, q<sub>i</sub>]</code> 表示第 `i` 个网络信号塔的坐标是 <code>(x<sub>i</sub>, y<sub>i</sub>)</code> 且信号强度参数为 <code>q<sub>i</sub></code> 。所有坐标都是在  X-Y 坐标系内的 **整数** 坐标。两个坐标之间的距离用 **欧几里得距离** 计算。

整数 `radius` 表示一个塔 **能到达** 的 **最远距离** 。如果一个坐标跟塔的距离在 `radius` 以内，那么该塔的信号可以到达该坐标。在这个范围以外信号会很微弱，所以 `radius` 以外的距离该塔是 **不能到达的** 。

如果第 `i` 个塔能到达 `(x, y)` ，那么该塔在此处的信号为 <code>⌊q<sub>i</sub> / (1 + d)⌋</code> ，其中 `d` 是塔跟此坐标的距离。一个坐标的 **信号强度** 是所有 **能到达** 该坐标的塔的信号强度之和。

请你返回数组 <code>[c<sub>x</sub>, c<sub>y</sub>]</code> ，表示 **信号强度** 最大的 **整数** 坐标点 <code>(c<sub>x</sub>, c<sub>y</sub>)</code> 。如果有多个坐标网络信号一样大，请你返回字典序最小的 **非负** 坐标。

**注意：**

* 坐标 `(x1, y1)` 字典序比另一个坐标 `(x2, y2)` 小，需满足以下条件之一：
    * 要么 `x1 < x2` ，
    * 要么 `x1 == x2` 且 `y1 < y2` 。
* `⌊val⌋` 表示小于等于 `val` 的最大整数（向下取整函数）。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/22/untitled-diagram.png)
<pre>
<strong>输入:</strong> towers = [[1,2,5],[2,1,7],[3,1,9]], radius = 2
<strong>输出:</strong> [2,1]
<strong>解释:</strong>
坐标 (2, 1) 信号强度之和为 13
- 塔 (2, 1) 强度参数为 7 ，在该点强度为 ⌊7 / (1 + sqrt(0)⌋ = ⌊7⌋ = 7
- 塔 (1, 2) 强度参数为 5 ，在该点强度为 ⌊5 / (1 + sqrt(2)⌋ = ⌊2.07⌋ = 2
- 塔 (3, 1) 强度参数为 9 ，在该点强度为 ⌊9 / (1 + sqrt(1)⌋ = ⌊4.5⌋ = 4
没有别的坐标有更大的信号强度。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> towers = [[23,11,21]], radius = 9
<strong>输出:</strong> [23,11]
<strong>解释:</strong> 由于仅存在一座信号塔，所以塔的位置信号强度最大。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> towers = [[1,2,13],[2,1,7],[0,1,9]], radius = 2
<strong>输出:</strong> [1,2]
<strong>解释:</strong> 坐标 (1, 2) 的信号强度最大。
</pre>

#### 提示:
* `1 <= towers.length <= 50`
* `towers[i].length == 3`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub>, q<sub>i</sub> <= 50</code>
* `1 <= radius <= 50`

## 题解 (Rust)

### 1. 题解
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
