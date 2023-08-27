# 699. 掉落的方块
在二维平面上的 x 轴上，放置着一些方块。

给你一个二维整数数组 `positions` ，其中 <code>positions[i] = [left<sub>i</sub>, sideLength<sub>i</sub>]</code> 表示：第 `i` 个方块边长为 <code>sideLength<sub>i</sub></code> ，其左侧边与 x 轴上坐标点 <code>left<sub>i</sub></code> 对齐。

每个方块都从一个比目前所有的落地方块更高的高度掉落而下。方块沿 y 轴负方向下落，直到着陆到 **另一个正方形的顶边** 或者是 **x 轴上** 。一个方块仅仅是擦过另一个方块的左侧边或右侧边不算着陆。一旦着陆，它就会固定在原地，无法移动。

在每个方块掉落后，你必须记录目前所有已经落稳的 **方块堆叠的最高高度** 。

返回一个整数数组 `ans` ，其中 `ans[i]` 表示在第 `i` 块方块掉落后堆叠的最高高度。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/04/28/fallingsq1-plane.jpg)
<pre>
<strong>输入:</strong> positions = [[1,2],[2,3],[6,1]]
<strong>输出:</strong> [2,5,5]
<strong>解释:</strong>
第 1 个方块掉落后，最高的堆叠由方块 1 组成，堆叠的最高高度为 2 。
第 2 个方块掉落后，最高的堆叠由方块 1 和 2 组成，堆叠的最高高度为 5 。
第 3 个方块掉落后，最高的堆叠仍然由方块 1 和 2 组成，堆叠的最高高度为 5 。
因此，返回 [2, 5, 5] 作为答案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> positions = [[100,100],[200,100]]
<strong>输出:</strong> [100,100]
<strong>解释:</strong>
第 1 个方块掉落后，最高的堆叠由方块 1 组成，堆叠的最高高度为 100 。
第 2 个方块掉落后，最高的堆叠可以由方块 1 组成也可以由方块 2 组成，堆叠的最高高度为 100 。
因此，返回 [100, 100] 作为答案。
注意，方块 2 擦过方块 1 的右侧边，但不会算作在方块 1 上着陆。
</pre>

#### 提示:
* `1 <= positions.length <= 1000`
* <code>1 <= left<sub>i</sub> <= 10<sup>8</sup></code>
* <code>1 <= sideLength<sub>i</sub> <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
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
