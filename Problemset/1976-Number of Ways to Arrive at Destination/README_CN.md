# 1976. 到达目的地的方案数
你在一个城市里，城市由 `n` 个路口组成，路口编号为 `0` 到 `n - 1` ，某些路口之间有 **双向** 道路。输入保证你可以从任意路口出发到达其他任意路口，且任意两个路口之间最多有一条路。

给你一个整数 `n` 和二维整数数组 `roads` ，其中 <code>roads[i] = [u<sub>i</sub>, v<sub>i</sub>, time<sub>i</sub>]</code> 表示在路口 <code>u<sub>i</sub></code> 和 <code>v<sub>i</sub></code> 之间有一条需要花费 <code>time<sub>i</sub></code> 时间才能通过的道路。你想知道花费 **最少时间** 从路口 `0` 出发到达路口 `n - 1` 的方案数。

请返回花费 **最少时间** 到达目的地的 **路径数目** 。由于答案可能很大，将结果对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2025/02/14/1976_corrected.png)
<pre>
<strong>输入:</strong> n = 7, roads = [[0,6,7],[0,1,2],[1,2,3],[1,3,3],[6,3,3],[3,5,1],[6,5,1],[2,5,1],[0,4,5],[4,6,2]]
<strong>输出:</strong> 4
<strong>解释:</strong> 从路口 0 出发到路口 6 花费的最少时间是 7 分钟。
四条花费 7 分钟的路径分别为：
- 0 ➝ 6
- 0 ➝ 4 ➝ 6
- 0 ➝ 1 ➝ 2 ➝ 5 ➝ 6
- 0 ➝ 1 ➝ 3 ➝ 5 ➝ 6
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2, roads = [[1,0,10]]
<strong>输出:</strong> 1
<strong>解释:</strong> 只有一条从路口 0 到路口 1 的路，花费 10 分钟。
</pre>

#### 提示:
* `1 <= n <= 200`
* `n - 1 <= roads.length <= n * (n - 1) / 2`
* `roads[i].length == 3`
* <code>0 <= u<sub>i</sub>, v<sub>i</sub> <= n - 1</code>
* <code>1 <= timei <= 10<sup>9</sup></code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* 任意两个路口之间至多有一条路。
* 从任意路口出发，你能够到达其他任意路口。

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut neighbors = vec![vec![]; n];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
        let mut min_time_count = vec![(i64::MAX, 0); n];
        min_time_count[0] = (0, 1);

        for road in &roads {
            neighbors[road[0] as usize].push((road[1] as usize, road[2] as i64));
            neighbors[road[1] as usize].push((road[0] as usize, road[2] as i64));
        }

        while let Some((Reverse(time), u)) = heap.pop() {
            if time > min_time_count[u].0 {
                continue;
            }

            for &(v, t) in &neighbors[u] {
                if time + t < min_time_count[v].0 {
                    min_time_count[v] = (time + t, min_time_count[u].1);
                    heap.push((Reverse(time + t), v));
                } else if time + t == min_time_count[v].0 {
                    min_time_count[v].1 =
                        (min_time_count[v].1 + min_time_count[u].1) % 1_000_000_007;
                }
            }
        }

        min_time_count[n - 1].1
    }
}
```
