# 1266. Minimum Time Visiting All Points
On a plane there are ```n``` points with integer coordinates ```points[i] = [xi, yi]```. Your task is to find the minimum time in seconds to visit all points.

You can move according to the next rules:
* In one second always you can either move vertically, horizontally by one unit or diagonally (it means to move one unit vertically and one unit horizontally in one second).
* You have to visit the points in the same order as they appear in the array.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/11/14/1626_example_1.PNG)
<pre>
<strong>Input:</strong> points = [[1,1],[3,4],[-1,0]]
<strong>Output:</strong> 7
<strong>Explanation:</strong> One optimal path is <strong>[1,1]</strong> -> [2,2] -> [3,3] -> <strong>[3,4]</strong> -> [2,3] -> [1,2] -> [0,1] -> <strong>[-1,0]</strong>
Time from [1,1] to [3,4] = 3 seconds
Time from [3,4] to [-1,0] = 4 seconds
Total time = 7 seconds
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> points = [[3,2],[-2,2]]
<strong>Output:</strong> 5
</pre>

#### Constraints:
* ```points.length == n```
* ```1 <= n <= 100```
* ```points[i].length == 2```
* ```-1000 <= points[i][0], points[i][1] <= 1000```

## Solutions (Rust)

### 1. Solution
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
