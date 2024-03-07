# 84. Largest Rectangle in Histogram
Given an array of integers `heights` representing the histogram's bar height where the width of each bar is `1`, return *the area of the largest rectangle in the histogram*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/01/04/histogram.jpg)
<pre>
<strong>Input:</strong> heights = [2,1,5,6,2,3]
<strong>Output:</strong> 10
<strong>Explanation:</strong> The above is a histogram where width of each bar is 1.
The largest rectangle is shown in the red area, which has an area = 10 units.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/01/04/histogram-1.jpg)
<pre>
<strong>Input:</strong> heights = [2,4]
<strong>Output:</strong> 4
</pre>

#### Constraints:
* <code>1 <= heights.length <= 10<sup>5</sup></code>
* <code>0 <= heights[i] <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut widths = vec![1; heights.len()];
        let mut ret = 0;

        for i in 0..heights.len() {
            while stack.last().unwrap_or(&(0, -1)).1 >= heights[i] {
                stack.pop();
            }

            widths[i] = i as i32 - stack.last().unwrap_or(&(-1, 0)).0;
            stack.push((i as i32, heights[i]));
        }

        stack.clear();

        for i in (0..heights.len()).rev() {
            while stack.last().unwrap_or(&(0, -1)).1 >= heights[i] {
                stack.pop();
            }

            widths[i] += stack.last().unwrap_or(&(heights.len() as i32, 0)).0 - 1 - i as i32;
            stack.push((i as i32, heights[i]));
            ret = ret.max(widths[i] * heights[i]);
        }

        ret
    }
}
```
