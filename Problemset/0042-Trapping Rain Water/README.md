# 42. Trapping Rain Water
Given `n` non-negative integers representing an elevation map where the width of each bar is `1`, compute how much water it can trap after raining.

#### Example 1:
![](https://assets.leetcode.com/uploads/2018/10/22/rainwatertrap.png)
<pre>
<strong>Input:</strong> height = [0,1,0,2,1,0,1,3,2,1,2,1]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> height = [4,2,0,3,2,5]
<strong>Output:</strong> 9
</pre>

#### Constraints:
* `n == height.length`
* <code>1 <= n <= 2 * 10<sup>4</sup></code>
* <code>0 <= height[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left_max = vec![0; height.len()];
        let mut right_max = vec![0; height.len()];

        for i in 1..height.len() {
            left_max[i] = left_max[i - 1].max(height[i - 1]);
        }
        for i in (0..height.len() - 1).rev() {
            right_max[i] = right_max[i + 1].max(height[i + 1]);
        }

        (0..height.len())
            .map(|i| (left_max[i].min(right_max[i]) - height[i]).max(0))
            .sum()
    }
}
```
