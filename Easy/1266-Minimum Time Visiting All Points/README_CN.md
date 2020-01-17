# 1266. 访问所有点的最小时间
平面上有 ```n``` 个点，点的位置用整数坐标表示 ```points[i] = [xi, yi]```。请你计算访问所有这些点需要的最小时间（以秒为单位）。

你可以按照下面的规则在平面上移动：
* 每一秒沿水平或者竖直方向移动一个单位长度，或者跨过对角线（可以看作在一秒内向水平和竖直方向各移动一个单位长度）。
* 必须按照数组中出现的顺序来访问这些点。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/11/24/1626_example_1.png)
<pre>
<strong>输入:</strong> points = [[1,1],[3,4],[-1,0]]
<strong>输出:</strong> 7
<strong>解释:</strong> 一条最佳的访问路径是： <strong>[1,1]</strong> -> [2,2] -> [3,3] -> <strong>[3,4]</strong> -> [2,3] -> [1,2] -> [0,1] -> <strong>[-1,0]</strong>
从 [1,1] 到 [3,4] 需要 3 秒
从 [3,4] 到 [-1,0] 需要 4 秒
一共需要 7 秒
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> points = [[3,2],[-2,2]]
<strong>输出:</strong> 5
</pre>

#### 提示:
* ```points.length == n```
* ```1 <= n <= 100```
* ```points[i].length == 2```
* ```-1000 <= points[i][0], points[i][1] <= 1000```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        for i in 0..(points.len() - 1) {
            let (x0, y0) = (points[i][0], points[i][1]);
            let (x1, y1) = (points[i + 1][0], points[i + 1][1]);
            ret += (x0 - x1).abs().max((y0 - y1).abs());
        }

        ret
    }
}
```
