# 699. Falling Squares
There are several squares being dropped onto the X-axis of a 2D plane.

You are given a 2D integer array `positions` where <code>positions[i] = [left<sub>i</sub>, sideLength<sub>i</sub>]</code> represents the <code>i<sup>th</sup></code> square with a side length of <code>sideLength<sub>i</sub></code> that is dropped with its left edge aligned with X-coordinate <code>left<sub>i</sub></code>.

Each square is dropped one at a time from a height above any landed squares. It then falls downward (negative Y direction) until it either lands **on the top side of another square** or **on the X-axis**. A square brushing the left/right side of another square does not count as landing on it. Once it lands, it freezes in place and cannot be moved.

After each square is dropped, you must record the **height of the current tallest stack of squares**.

Return *an integer array* `ans` *where* `ans[i]` *represents the height described above after dropping the* <code>i<sup>th</sup></code> *square*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/04/28/fallingsq1-plane.jpg)
<pre>
<strong>Input:</strong> positions = [[1,2],[2,3],[6,1]]
<strong>Output:</strong> [2,5,5]
<strong>Explanation:</strong>
After the first drop, the tallest stack is square 1 with a height of 2.
After the second drop, the tallest stack is squares 1 and 2 with a height of 5.
After the third drop, the tallest stack is still squares 1 and 2 with a height of 5.
Thus, we return an answer of [2, 5, 5].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> positions = [[100,100],[200,100]]
<strong>Output:</strong> [100,100]
<strong>Explanation:</strong>
After the first drop, the tallest stack is square 1 with a height of 100.
After the second drop, the tallest stack is either square 1 or square 2, both with heights of 100.
Thus, we return an answer of [100, 100].
Note that square 2 only brushes the right side of square 1, which does not count as landing on it.
</pre>

#### Constraints:
* `1 <= positions.length <= 1000`
* <code>1 <= left<sub>i</sub> <= 10<sup>8</sup></code>
* <code>1 <= sideLength<sub>i</sub> <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut heights = vec![(
            positions[0][0],
            positions[0][0] + positions[0][1],
            positions[0][1],
        )];
        let mut ans = vec![0; positions.len()];
        ans[0] = positions[0][1];

        for i in 1..positions.len() {
            let left = positions[i][0];
            let length = positions[i][1];
            let right = left + length;
            let mut height = length;

            for j in 0..heights.len() {
                if heights[j].1 > left && heights[j].0 < right {
                    height = height.max(heights[j].2 + length);
                }
            }

            heights.push((left, right, height));
            ans[i] = ans[i - 1].max(height);
        }

        ans
    }
}
```
