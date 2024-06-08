# 1615. 最大网络秩
`n` 座城市和一些连接这些城市的道路 `roads` 共同组成一个基础设施网络。每个 <code>roads[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 都表示在城市 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间有一条双向道路。

两座不同城市构成的 **城市对** 的 **网络秩** 定义为：与这两座城市 **直接** 相连的道路总数。如果存在一条道路直接连接这两座城市，则这条道路只计算 **一次** 。

整个基础设施网络的 **最大网络秩** 是所有不同城市对中的 **最大网络秩** 。

给你整数 `n` 和数组 `roads`，返回整个基础设施网络的 **最大网络秩** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/21/ex1.png)
<pre>
<strong>输入:</strong> n = 4, roads = [[0,1],[0,3],[1,2],[1,3]]
<strong>输出:</strong> 4
<strong>解释:</strong> 城市 0 和 1 的网络秩是 4，因为共有 4 条道路与城市 0 或 1 相连。位于 0 和 1 之间的道路只计算一次。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/09/21/ex2.png)
<pre>
<strong>输入:</strong> n = 5, roads = [[0,1],[0,3],[1,2],[1,3],[2,3],[2,4]]
<strong>输出:</strong> 5
<strong>解释:</strong> 共有 5 条道路与城市 1 或 2 相连。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 8, roads = [[0,1],[1,2],[2,3],[2,4],[5,6],[5,7]]
<strong>输出:</strong> 5
<strong>解释:</strong> 2 和 5 的网络秩为 5，注意并非所有的城市都需要连接起来。
</pre>

#### 提示:
* `2 <= n <= 100`
* `0 <= roads.length <= n * (n - 1) / 2`
* `roads[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= n-1</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* 每对城市之间 **最多只有一条** 道路相连

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut roads_set = HashSet::new();
        let mut count = vec![0; n];
        let mut ret = 0;

        for road in &roads {
            let a = (road[0]).min(road[1]) as usize;
            let b = (road[0]).max(road[1]) as usize;

            roads_set.insert((a, b));
            count[a] += 1;
            count[b] += 1;
        }

        for a in 0..n {
            for b in a + 1..n {
                ret = ret.max(count[a] + count[b] - roads_set.contains(&(a, b)) as i32);
            }
        }

        ret
    }
}
```
