# 2492. 两个城市间路径的最小分数
给你一个正整数 `n` ，表示总共有 `n` 个城市，城市从 `1` 到 `n` 编号。给你一个二维数组 `roads` ，其中 <code>roads[i] = [a<sub>i</sub>, b<sub>i</sub>, distance<sub>i</sub>]</code> 表示城市 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间有一条 **双向** 道路，道路距离为 <code>distance<sub>i</sub></code> 。城市构成的图不一定是连通的。

两个城市之间一条路径的 **分数** 定义为这条路径中道路的 **最小** 距离。

城市 `1` 和城市 `n` 之间的所有路径的 **最小** 分数。

**注意：**
* 一条路径指的是两个城市之间的道路序列。
* 一条路径可以 **多次** 包含同一条道路，你也可以沿着路径多次到达城市 `1` 和城市 `n` 。
* 测试数据保证城市 `1` 和城市`n` 之间 **至少** 有一条路径。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/10/12/graph11.png)
<pre>
<strong>输入:</strong> n = 4, roads = [[1,2,9],[2,3,6],[2,4,5],[1,4,7]]
<strong>输出:</strong> 5
<strong>解释:</strong> 城市 1 到城市 4 的路径中，分数最小的一条为：1 -> 2 -> 4 。这条路径的分数是 min(9,5) = 5 。
不存在分数更小的路径。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/10/12/graph22.png)
<pre>
<strong>输入:</strong> n = 4, roads = [[1,2,2],[1,3,4],[3,4,7]]
<strong>输出:</strong> 2
<strong>解释:</strong> 城市 1 到城市 4 分数最小的路径是：1 -> 2 -> 1 -> 3 -> 4 。这条路径的分数是 min(2,2,4,7) = 2 。
</pre>

#### 提示:
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>1 <= roads.length <= 10<sup>5</sup></code>
* `roads[i].length == 3`
* <code>1 <= a<sub>i</sub>, b<sub>i</sub> <= n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* <code>1 <= distancei <= 10<sup>4</sup></code>
* 不会有重复的边。
* 城市 `1` 和城市 `n` 之间至少有一条路径。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut neighbors = vec![vec![]; n as usize + 1];
        let mut stack = vec![1];
        let mut visited = vec![false; n as usize + 1];
        visited[1] = true;
        let mut ret = i32::MAX;

        for road in &roads {
            let (a, b, distance) = (road[0] as usize, road[1] as usize, road[2]);
            neighbors[a].push((b, distance));
            neighbors[b].push((a, distance));
        }

        while let Some(a) = stack.pop() {
            for &(b, distance) in &neighbors[a] {
                if !visited[b] {
                    stack.push(b);
                    visited[b] = true;
                }
                ret = ret.min(distance);
            }
        }

        ret
    }
}
```
