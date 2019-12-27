# 812. 最大三角形面积
给定包含多个点的集合，从其中取三个点组成三角形，返回能组成的最大三角形的面积。

<pre>
<strong>示例:</strong>
<strong>输入:</strong> points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
<strong>输出:</strong> 2
<strong>解释:</strong>
这五个点如下图所示。组成的橙色三角形是最大的，面积为2。
</pre>

![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/04/1027.png)

#### 注意:
* ```3 <= points.length <= 50```.
* 不存在重复的点。
* ``` -50 <= points[i][j] <= 50```.
* 结果误差值在 ```10^-6``` 以内都认为是正确答案。

## 题解 (Rust)

### 1. 暴力法
```Rust
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut max_area = 0_f64;

        for i in 0..(points.len() - 2) {
            let a = &points[i];
            for j in (i + 1)..(points.len() - 1) {
                let b = &points[j];
                for k in (j + 1)..points.len() {
                    let c = &points[k];
                    max_area = max_area.max(Self::area(a, b, c));
                }
            }
        }

        max_area
    }
    
    pub fn area(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>) -> f64 {
        (a[0] * b[1] + b[0] * c[1] + c[0] * a[1] -
        a[1] * b[0] - b[1] * c[0] - c[1] * a[0]).abs() as f64 / 2.0
    }
}
```
