# 973. K Closest Points to Origin
We have a list of ```points``` on the plane.  Find the ```K``` closest points to the origin ```(0, 0)```.

(Here, the distance between two points on a plane is the Euclidean distance.)

You may return the answer in any order.  The answer is guaranteed to be unique (except for the order that it is in.)

#### Example 1:
<pre>
<strong>Input:</strong> points = [[1,3],[-2,2]], K = 1
<strong>Output:</strong> [[-2,2]]
<strong>Explanation:</strong>
The distance between (1, 3) and the origin is sqrt(10).
The distance between (-2, 2) and the origin is sqrt(8).
Since sqrt(8) < sqrt(10), (-2, 2) is closer to the origin.
We only want the closest K = 1 points from the origin, so the answer is just [[-2,2]].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> points = [[3,3],[5,-1],[-2,4]], K = 2
<strong>Output:</strong> [[3,3],[-2,4]]
(The answer [[-2,4],[3,3]] would also be accepted.)
</pre>

#### Note:
1. ```1 <= K <= points.length <= 10000```
2. ```-10000 < points[i][0] < 10000```
3. ```-10000 < points[i][1] < 10000```

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points;
        points.sort_unstable_by_key(|p| p[0] * p[0] + p[1] * p[1]);
        points[..(k as usize)].to_vec()
    }
}
```
