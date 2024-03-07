# 84. 柱状图中最大的矩形
给定 *n* 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。

求在该柱状图中，能够勾勒出来的矩形的最大面积。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/01/04/histogram.jpg)
<pre>
<strong>输入:</strong> heights = [2,1,5,6,2,3]
<strong>输出:</strong> 10
<strong>解释:</strong> 最大的矩形为图中红色区域，面积为 10
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/01/04/histogram-1.jpg)
<pre>
<strong>输入:</strong> heights = [2,4]
<strong>输出:</strong> 4
</pre>

#### 提示:
* <code>1 <= heights.length <= 10<sup>5</sup></code>
* <code>0 <= heights[i] <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
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
