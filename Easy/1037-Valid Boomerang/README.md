# 1037. Valid Boomerang
A *boomerang* is a set of 3 points that are all distinct and **not** in a straight line.

Given a list of three points in the plane, return whether these points are a boomerang.

#### Example 1:
<pre>
<strong>Input:</strong> [[1,1],[2,3],[3,2]]
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [[1,1],[2,2],[3,3]]
<strong>Output:</strong> false
</pre>

#### Note:
1. ```points.length == 3```
2. ```points[i].length == 2```
3. ```0 <= points[i][j] <= 100```

## Solutions (Rust)

### 1. Linear Equation
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

### 2. Triangle Area
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

### 3. Triangle Inequality
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
