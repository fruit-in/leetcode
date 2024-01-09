# 502. IPO
假设 力扣（LeetCode）即将开始 **IPO** 。为了以更高的价格将股票卖给风险投资公司，力扣 希望在 IPO 之前开展一些项目以增加其资本。 由于资源有限，它只能在 IPO 之前完成最多 `k` 个不同的项目。帮助 力扣 设计完成最多 `k` 个不同项目后得到最大总资本的方式。

给你 `n` 个项目。对于每个项目 `i` ，它都有一个纯利润 `profits[i]` ，和启动该项目需要的最小资本 `capital[i]` 。

最初，你的资本为 `w` 。当你完成一个项目时，你将获得纯利润，且利润将被添加到你的总资本中。

总而言之，从给定项目中选择 **最多** `k` 个不同项目的列表，以 **最大化最终资本** ，并输出最终可获得的最多资本。

答案保证在 32 位有符号整数范围内。

#### 示例 1:
<pre>
<strong>输入:</strong> k = 2, w = 0, profits = [1,2,3], capital = [0,1,1]
<strong>输出:</strong> 4
<strong>解释:</strong>
由于你的初始资本为 0，你仅可以从 0 号项目开始。
在完成后，你将获得 1 的利润，你的总资本将变为 1。
此时你可以选择开始 1 号或 2 号项目。
由于你最多可以选择两个项目，所以你需要完成 2 号项目以获得最大的资本。
因此，输出最后最大化的资本，为 0 + 1 + 3 = 4。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> k = 3, w = 0, profits = [1,2,3], capital = [0,1,2]
<strong>输出:</strong> 6
</pre>

#### 提示:
* <code>1 <= k <= 10<sup>5</sup></code>
* <code>0 <= w <= 10<sup>9</sup></code>
* `n == profits.length`
* `n == capital.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= profits[i] <= 10<sup>4</sup></code>
* <code>0 <= capital[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut w = w;
        let mut projects = capital
            .into_iter()
            .zip(profits.into_iter())
            .collect::<Vec<_>>();
        let mut heap = BinaryHeap::new();
        let mut i = 0;

        projects.sort_unstable();

        for _ in 0..k {
            while i < projects.len() && projects[i].0 <= w {
                heap.push(projects[i].1);
                i += 1;
            }

            w += heap.pop().unwrap_or(0);
        }

        w
    }
}
```
