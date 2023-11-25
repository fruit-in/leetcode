# 2106. 摘水果
在一个无限的 x 坐标轴上，有许多水果分布在其中某些位置。给你一个二维整数数组 `fruits` ，其中 <code>fruits[i] = [position<sub>i</sub>, amount<sub>i</sub>]</code> 表示共有 <code>amount<sub>i</sub></code> 个水果放置在 <code>position<sub>i</sub></code> 上。`fruits` 已经按 <code>position<sub>i</sub></code> **升序排列** ，每个 <code>position<sub>i</sub></code> **互不相同** 。

另给你两个整数 `startPos` 和 `k` 。最初，你位于 `startPos` 。从任何位置，你可以选择 **向左或者向右** 走。在 x 轴上每移动 **一个单位** ，就记作 **一步** 。你总共可以走 **最多** `k` 步。你每达到一个位置，都会摘掉全部的水果，水果也将从该位置消失（不会再生）。

返回你可以摘到水果的 **最大总数** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/11/21/1.png)
<pre>
<strong>输入:</strong> fruits = [[2,8],[6,3],[8,6]], startPos = 5, k = 4
<strong>输出:</strong> 9
<strong>解释:</strong>
最佳路线为：
- 向右移动到位置 6 ，摘到 3 个水果
- 向右移动到位置 8 ，摘到 6 个水果
移动 3 步，共摘到 3 + 6 = 9 个水果
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/11/21/2.png)
<pre>
<strong>输入:</strong> fruits = [[0,9],[4,1],[5,7],[6,2],[7,4],[10,9]], startPos = 5, k = 4
<strong>输出:</strong> 14
<strong>解释:</strong>
可以移动最多 k = 4 步，所以无法到达位置 0 和位置 10 。
最佳路线为：
- 在初始位置 5 ，摘到 7 个水果
- 向左移动到位置 4 ，摘到 1 个水果
- 向右移动到位置 6 ，摘到 2 个水果
- 向右移动到位置 7 ，摘到 4 个水果
移动 1 + 3 = 4 步，共摘到 7 + 1 + 2 + 4 = 14 个水果
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/11/21/3.png)
<pre>
<strong>输入:</strong> fruits = [[0,3],[6,4],[8,5]], startPos = 3, k = 2
<strong>输出:</strong> 0
<strong>解释:</strong>
最多可以移动 k = 2 步，无法到达任一有水果的地方
</pre>

#### 提示:
* <code>1 <= fruits.length <= 10<sup>5</sup></code>
* `fruits[i].length == 2`
* <code>0 <= startPos, position<sub>i</sub> <= 2 * 10<sup>5</sup></code>
* 对于任意 `i > 0` ，<code>position<sup>i</sup>-1 < position<sup>i</sup></code> 均成立（下标从 **0** 开始计数）
* <code>1 <= amount<sub>i</sub> <= 10<sup>4</sup></code>
* <code>0 <= k <= 2 * 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut prefix_sum = vec![(-1, 0)];
        let mut ret = 0;

        for fruit in &fruits {
            if (fruit[0] - start_pos).abs() <= k {
                let amount = prefix_sum.last().unwrap().1;
                prefix_sum.push((fruit[0], fruit[1] + amount));
            }
        }

        for i in 1..prefix_sum.len() {
            if prefix_sum[i].0 < start_pos {
                let start = prefix_sum[i].0;
                let end = start_pos.max(k - start_pos + 2 * start);
                let j = prefix_sum.binary_search(&(end, i32::MAX)).unwrap_err() - 1;

                ret = ret.max(prefix_sum[j].1 - prefix_sum[i - 1].1);
            } else {
                let end = prefix_sum[i].0;
                let start = start_pos.min(2 * end - k - start_pos).max(0);
                let j = prefix_sum.binary_search(&(start, 0)).unwrap_err();

                ret = ret.max(prefix_sum[i].1 - prefix_sum[j - 1].1);
            }
        }

        ret
    }
}
```
