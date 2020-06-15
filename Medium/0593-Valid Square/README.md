# 593. Valid Square
Given the coordinates of four points in 2D space, return whether the four points could construct a square.

The coordinate (x,y) of a point is represented by an integer array with two integers.

#### Example:
<pre>
<strong>Input:</strong> p1 = [0,0], p2 = [1,1], p3 = [1,0], p4 = [0,1]
<strong>Output:</strong> True
</pre>

#### Note:
1. All the input integers are in the range [-10000, 10000].
2. A valid square has four equal sides with positive length and four equal angles (90-degree angles).
3. Input points have no order.

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let points = [p1, p2, p3, p4];
        let mut dis_2 = Vec::new();

        for i in 1..4 {
            for j in 0..i {
                let x = points[i][0] - points[j][0];
                let y = points[i][1] - points[j][1];
                dis_2.push(x * x + y * y);
            }
        }

        dis_2.sort_unstable();

        dis_2[0] != 0 && dis_2[0] == dis_2[3] && dis_2[4] == dis_2[5]
    }
}
```
