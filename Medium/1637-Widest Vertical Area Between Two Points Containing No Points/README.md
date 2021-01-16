# 1637. Widest Vertical Area Between Two Points Containing No Points
Given `n` `points` on a 2D plane where <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code>, Return *the **widest vertical area** between two points such that no points are inside the area*.

A **vertical area** is an area of fixed-width extending infinitely along the y-axis (i.e., infinite height). The **widest vertical area** is the one with the maximum width.

Note that points **on the edge** of a vertical area **are not** considered included in the area.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/19/points3.png)
<pre>
<strong>Input:</strong> points = [[8,7],[9,9],[7,4],[9,7]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Both the red and the blue area are optimal.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> points = [[3,1],[9,0],[1,0],[1,4],[5,3],[8,8]]
<strong>Output:</strong> 3
</pre>

#### Constraints:
* `n == points.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `points[i].length == 2`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>9</sup></code>

## Solutions (Ruby)

### 1. Sort
```Ruby
# @param {Integer[][]} points
# @return {Integer}
def max_width_of_vertical_area(points)
  ret = 0

  points.sort_by! { |p| p[0] }

  (1...points.length).each do |i|
    ret = [ret, points[i][0] - points[i - 1][0]].max
  end

  ret
end
```

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        points.sort_unstable_by_key(|p| p[0]);

        for i in 1..points.len() {
            ret = ret.max(points[i][0] - points[i - 1][0]);
        }

        ret
    }
}
```
