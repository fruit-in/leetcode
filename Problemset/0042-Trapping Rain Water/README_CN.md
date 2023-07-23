# 42. 接雨水
给定 `n` 个非负整数表示每个宽度为 `1` 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2018/10/22/rainwatertrap.png)
<pre>
<strong>输入:</strong> height = [0,1,0,2,1,0,1,3,2,1,2,1]
<strong>输出:</strong> 6
<strong>解释:</strong> 上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> height = [4,2,0,3,2,5]
<strong>输出:</strong> 9
</pre>

#### 提示:
* `n == height.length`
* <code>1 <= n <= 2 * 10<sup>4</sup></code>
* <code>0 <= height[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
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
