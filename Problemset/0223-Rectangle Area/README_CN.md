# 223. 矩形面积
在**二维**平面上计算出两个**由直线构成的**矩形重叠后形成的总面积。

每个矩形由其左下顶点和右上顶点坐标表示，如图所示。

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/22/rectangle_area.png)

#### 示例:
<pre>
<strong>Input:</strong> -3, 0, 3, 4, 0, -1, 9, 2
<strong>Output:</strong> 45
</pre>

**说明:** 假设矩形面积不会超出 **int** 的范围。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer} a
# @param {Integer} b
# @param {Integer} c
# @param {Integer} d
# @param {Integer} e
# @param {Integer} f
# @param {Integer} g
# @param {Integer} h
# @return {Integer}
def compute_area(a, b, c, d, e, f, g, h)
    area0 = (c - a) * (d - b)
    area1 = (g - e) * (h - f)
    area2 = [[c, g].min - [a, e].max, 0].max * [[d, h].min - [b, f].max, 0].max

    area0 + area1 - area2
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        let area0 = (c - a) * (d - b);
        let area1 = (g - e) * (h - f);
        let area2 = (c.min(g).saturating_sub(a.max(e))).max(0) *
            (d.min(h).saturating_sub(b.max(f))).max(0);

        area0 + area1 - area2
    }
}
```
