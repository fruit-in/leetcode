# 1637. 两点之间不包含任何点的最宽垂直面积
给你 `n` 个二维平面上的点 `points` ，其中 <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> ，请你返回两点之间内部不包含任何点的 **最宽垂直面积** 的宽度。

**垂直面积** 的定义是固定宽度，而 y 轴上无限延伸的一块区域（也就是高度为无穷大）。 **最宽垂直面积** 为宽度最大的一个垂直面积。

请注意，垂直区域 **边上** 的点 **不在** 区域内。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/10/31/points3.png)
<pre>
<strong>输入:</strong> points = [[8,7],[9,9],[7,4],[9,7]]
<strong>输出:</strong> 1
<strong>解释:</strong> 红色区域和蓝色区域都是最优区域。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> points = [[3,1],[9,0],[1,0],[1,4],[5,3],[8,8]]
<strong>输出:</strong> 3
</pre>

#### 提示:
* `n == points.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `points[i].length == 2`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>9</sup></code>

## 题解 (Ruby)

### 1. 排序
```Ruby
# @param {Integer[][]} points
# @return {Integer}
def max_width_of_vertical_area(points)
  ret = 0

  points.sort_by! { |p| p[0] }

  (1...points.length).each do |i|
    ret = [ret, points[i][0] - points[i - 1][0]].max
  end

  ret
end
```

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        points.sort_unstable_by_key(|p| p[0]);

        for i in 1..points.len() {
            ret = ret.max(points[i][0] - points[i - 1][0]);
        }

        ret
    }
}
```
