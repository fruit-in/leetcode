# 452. Minimum Number of Arrows to Burst Balloons
There are some spherical balloons spread in two-dimensional space. For each balloon, provided input is the start and end coordinates of the horizontal diameter. Since it's horizontal, y-coordinates don't matter, and hence the x-coordinates of start and end of the diameter suffice. The start is always smaller than the end.

An arrow can be shot up exactly vertically from different points along the x-axis. A balloon with <code>x<sub>start</sub></code> and <code>x<sub>end</sub></code> bursts by an arrow shot at `x` if <code>x<sub>start</sub> ≤ x ≤ x<sub>end</sub></code>. There is no limit to the number of arrows that can be shot. An arrow once shot keeps traveling up infinitely.

Given an array `points` where <code>points[i] = [x<sub>start</sub>, x<sub>end</sub>]</code>, return *the minimum number of arrows that must be shot to burst all balloons*.

#### Example 1:
<pre>
<strong>Input:</strong> points = [[10,16],[2,8],[1,6],[7,12]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> One way is to shoot one arrow for example at x = 6 (bursting the balloons [2,8] and [1,6]) and another arrow at x = 11 (bursting the other two balloons).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> points = [[1,2],[3,4],[5,6],[7,8]]
<strong>Output:</strong> 4
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> points = [[1,2],[2,3],[3,4],[4,5]]
<strong>Output:</strong> 2
</pre>

#### Constraints:
* <code>1 <= points.length <= 10<sup>4</sup></code>
* `points[i].length == 2`
* <code>-2<sup>31</sup> <= x<sub>start</sub> < x<sub>end</sub> <= 2<sup>31</sup> - 1</code>

## Solutions (Ruby)

### 1. Sort
```Ruby
# @param {Integer[][]} points
# @return {Integer}
def find_min_arrow_shots(points)
  points.sort_by! { |p| p[1] }
  x = points[0][1]
  ret = 1

  (1...points.size).each do |i|
    if points[i][0] > x
      x = points[i][1]
      ret += 1
    end
  end

  ret
end
```

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|p| p[1]);
        let mut x = points[0][1];
        let mut ret = 1;

        for i in 1..points.len() {
            if points[i][0] > x {
                x = points[i][1];
                ret += 1;
            }
        }

        ret
    }
}
```
