# 1401. 圆和矩形是否有重叠
给你一个以 (`radius`, `x_center`, `y_center`) 表示的圆和一个与坐标轴平行的矩形 (`x1`, `y1`, `x2`, `y2`)，其中 (`x1`, `y1`) 是矩形左下角的坐标，(`x2`, `y2`) 是右上角的坐标。

如果圆和矩形有重叠的部分，请你返回 True ，否则返回 False 。

换句话说，请你检测是否 **存在** 点 (xi, yi) ，它既在圆上也在矩形上（两者都包括点落在边界上的情况）。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/04/04/sample_4_1728.png)
<pre>
<strong>输入:</strong> radius = 1, x_center = 0, y_center = 0, x1 = 1, y1 = -1, x2 = 3, y2 = 1
<strong>输出:</strong> true
<strong>解释:</strong> 圆和矩形有公共点 (1,0)
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/04/04/sample_2_1728.png)
<pre>
<strong>输入:</strong> radius = 1, x_center = 0, y_center = 0, x1 = -1, y1 = 0, x2 = 0, y2 = 1
<strong>输出:</strong> true
</pre>

#### 示例 3:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/04/04/sample_6_1728.png)
<pre>
<strong>输入:</strong> radius = 1, x_center = 1, y_center = 1, x1 = -3, y1 = -3, x2 = 3, y2 = 3
<strong>输出:</strong> true
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> radius = 1, x_center = 1, y_center = 1, x1 = 1, y1 = -3, x2 = 2, y2 = -1
<strong>输出:</strong> false
</pre>

#### 提示:
* `1 <= radius <= 2000`
* `-10^4 <= x_center, y_center, x1, y1, x2, y2 <= 10^4`
* `x1 < x2`
* `y1 < y2`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let x = if x_center >= x1 && x_center <= x2 {
            0
        } else {
            (x_center - x1).abs().min((x_center - x2).abs())
        };
        let y = if y_center >= y1 && y_center <= y2 {
            0
        } else {
            (y_center - y1).abs().min((y_center - y2).abs())
        };

        x * x + y * y <= radius * radius
    }
}
```
