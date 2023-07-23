# 1232. 缀点成线
在一个 XY 坐标系中有一些点，我们用数组 ```coordinates``` 来分别记录它们的坐标，其中 ```coordinates[i] = [x, y]``` 表示横坐标为 ```x```、纵坐标为 ```y``` 的点。

请你来判断，这些点是否在该坐标系中属于同一条直线上，是则返回 ```true```，否则请返回 ```false```。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/10/19/untitled-diagram-2.jpg)
<pre>
<strong>输入:</strong> coordinates = [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]
<strong>输出:</strong> true
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/10/19/untitled-diagram-1.jpg)
<pre>
<strong>输入:</strong> coordinates = [[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]]
<strong>输出:</strong> false
</pre>

#### 提示:
* ```2 <= coordinates.length <= 1000```
* ```coordinates[i].length == 2```
* ```-10^4 <= coordinates[i][0], coordinates[i][1] <= 10^4```
* ```coordinates``` 中不含重复的点

## 题解 (Rust)

### 1. 直线方程
```Rust
impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let x0 = &coordinates[0][0];
        let y0 = &coordinates[0][1];
        let x1 = &coordinates[1][0];
        let y1 = &coordinates[1][1];

        for point in &coordinates[2..] {
            let x = point[0];
            let y = point[1];

            if (y - y0) * (x1 - x0) != (x - x0) * (y1 - y0) {
                return false;
            }
        }

        true
    }
}
```
