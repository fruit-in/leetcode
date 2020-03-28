# 973. 最接近原点的 K 个点
我们有一个由平面上的点组成的列表 ```points```。需要从中找出 ```K``` 个距离原点 ```(0, 0)``` 最近的点。

（这里，平面上两点之间的距离是欧几里德距离。）

你可以按任何顺序返回答案。除了点坐标的顺序之外，答案确保是唯一的。

#### 示例 1:
<pre>
<strong>输入:</strong> points = [[1,3],[-2,2]], K = 1
<strong>输出:</strong> [[-2,2]]
<strong>解释:</strong>
(1, 3) 和原点之间的距离为 sqrt(10)，
(-2, 2) 和原点之间的距离为 sqrt(8)，
由于 sqrt(8) < sqrt(10)，(-2, 2) 离原点更近。
我们只需要距离原点最近的 K = 1 个点，所以答案就是 [[-2,2]]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> points = [[3,3],[5,-1],[-2,4]], K = 2
<strong>输出:</strong> [[3,3],[-2,4]]
（答案 [[-2,4],[3,3]] 也会被接受。）
</pre>

#### 提示:
1. ```1 <= K <= points.length <= 10000```
2. ```-10000 < points[i][0] < 10000```
3. ```-10000 < points[i][1] < 10000```

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points;
        points.sort_unstable_by_key(|p| p[0] * p[0] + p[1] * p[1]);
        points[..(k as usize)].to_vec()
    }
}
```
