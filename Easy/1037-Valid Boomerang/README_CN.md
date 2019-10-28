# 1037. 有效的回旋镖
回旋镖定义为一组三个点，这些点各不相同且**不**在一条直线上。

给出平面上三个点组成的列表，判断这些点是否可以构成回旋镖。

#### 示例 1:
<pre>
<strong>输入:</strong> [[1,1],[2,3],[3,2]]
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [[1,1],[2,2],[3,3]]
<strong>输出:</strong> false
</pre>

#### 提示:
1. ```points.length == 3```
2. ```points[i].length == 2```
3. ```0 <= points[i][j] <= 100```

## 题解 (Rust)

### 1. 直线方程
```Rust
impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let x0 = points[0][0];
        let y0 = points[0][1];
        let x1 = points[1][0];
        let y1 = points[1][1];
        let x2 = points[2][0];
        let y2 = points[2][1];

        (x1 - x0) * (y2 - y0) != (x2 - x0) * (y1 - y0)
    }
}
```

### 2. 三角形面积
```Rust
impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let x0 = points[0][0];
        let y0 = points[0][1];
        let x1 = points[1][0];
        let y1 = points[1][1];
        let x2 = points[2][0];
        let y2 = points[2][1];

        x0 * y1 + x1 * y2 + x2 * y0 - y0 * x1 - y1 * x2 - y2 * x0 != 0
    }
}
```

### 3. 三角不等式
```Rust
impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let x0 = points[0][0] as f64;
        let y0 = points[0][1] as f64;
        let x1 = points[1][0] as f64;
        let y1 = points[1][1] as f64;
        let x2 = points[2][0] as f64;
        let y2 = points[2][1] as f64;

        let a = ((x1 - x0) * (x1 - x0) + (y1 - y0) * (y1 - y0)).sqrt();
        let b = ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt();
        let c = ((x2 - x0) * (x2 - x0) + (y2 - y0) * (y2 - y0)).sqrt();

        a + b > c && b + c > a && a + c > b
    }
}
```
