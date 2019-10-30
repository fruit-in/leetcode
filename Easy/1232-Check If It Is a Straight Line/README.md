# 1232. Check If It Is a Straight Line
You are given an array ```coordinates```, ```coordinates[i] = [x, y]```, where ```[x, y]``` represents the coordinate of a point. Check if these points make a straight line in the XY plane.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/10/15/untitled-diagram-2.jpg)
<pre>
<strong>Input:</strong> coordinates = [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]
<strong>Output:</strong> true
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/10/09/untitled-diagram-1.jpg)
<pre>
<strong>Input:</strong> coordinates = [[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]]
<strong>Output:</strong> false
</pre>

#### Constraints:
* ```2 <= coordinates.length <= 1000```
* ```coordinates[i].length == 2```
* ```-10^4 <= coordinates[i][0], coordinates[i][1] <= 10^4```
* ```coordinates``` contains no duplicate point.

## Solutions (Rust)

### 1. Linear Equation
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
