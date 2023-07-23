# 1642. 可以到达的最远建筑
给你一个整数数组 `heights` ，表示建筑物的高度。另有一些砖块 `bricks` 和梯子 `ladders` 。

你从建筑物 `0` 开始旅程，不断向后面的建筑物移动，期间可能会用到砖块或梯子。

当从建筑物 `i` 移动到建筑物 `i+1`（下标 **从 0 开始** ）时：

* 如果当前建筑物的高度 **大于或等于** 下一建筑物的高度，则不需要梯子或砖块
* 如果当前建筑的高度 小于 下一个建筑的高度，您可以使用 **一架梯子** 或 **`(h[i+1] - h[i])` 个砖块**

如果以最佳方式使用给定的梯子和砖块，返回你可以到达的最远建筑物的下标（下标 **从 0 开始** ）。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/10/27/q4.gif)
<pre>
<strong>输入:</strong> heights = [4,2,7,6,9,14,12], bricks = 5, ladders = 1
<strong>输出:</strong> 4
<strong>解释:</strong> 从建筑物 0 出发，你可以按此方案完成旅程：
- 不使用砖块或梯子到达建筑物 1 ，因为 4 >= 2
- 使用 5 个砖块到达建筑物 2 。你必须使用砖块或梯子，因为 2 < 7
- 不使用砖块或梯子到达建筑物 3 ，因为 7 >= 6
- 使用唯一的梯子到达建筑物 4 。你必须使用砖块或梯子，因为 6 < 9
无法越过建筑物 4 ，因为没有更多砖块或梯子。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> heights = [4,12,2,7,3,18,20,3,19], bricks = 10, ladders = 2
<strong>输出:</strong> 7
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> heights = [14,3,19,3], bricks = 17, ladders = 0
<strong>输出:</strong> 3
</pre>

#### 提示:
* <code>1 <= heights.length <= 10<sup>5</sup></code>
* <code>1 <= heights[i] <= 10<sup>6</sup></code>
* <code>0 <= bricks <= 10<sup>9</sup></code>
* `0 <= ladders <= heights.length`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut bricks = bricks;
        let mut ladders = ladders;
        let mut heap = BinaryHeap::new();

        for i in 0..heights.len() - 1 {
            let diff = heights[i + 1] - heights[i];

            if diff > 0 {
                heap.push(diff);

                if bricks < diff {
                    if ladders > 0 {
                        bricks += heap.pop().unwrap();
                        ladders -= 1;
                    } else {
                        return i as i32;
                    }
                }

                bricks -= diff;
            }
        }

        heights.len() as i32 - 1
    }
}
```
