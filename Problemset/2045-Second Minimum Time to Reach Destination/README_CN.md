# 2045. 到达目的地的第二短时间
城市用一个 **双向连通** 图表示，图中有 `n` 个节点，从 `1` 到 `n` 编号（包含 `1` 和 `n`）。图中的边用一个二维整数数组 `edges` 表示，其中每个 <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> 表示一条节点 <code>u<sub>i</sub></code> 和节点 <code>v<sub>i</sub></code> 之间的双向连通边。每组节点对由 **最多一条** 边连通，顶点不存在连接到自身的边。穿过任意一条边的时间是 `time` 分钟。

每个节点都有一个交通信号灯，每 `change` 分钟改变一次，从绿色变成红色，再由红色变成绿色，循环往复。所有信号灯都 **同时** 改变。你可以在 **任何时候** 进入某个节点，但是 **只能** 在节点 **信号灯是绿色时** 才能离开。如果信号灯是  **绿色** ，你 **不能** 在节点等待，必须离开。

**第二小的值** 是 **严格大于** 最小值的所有值中最小的值。

* 例如，`[2, 3, 4]` 中第二小的值是 `3` ，而 `[2, 2, 4]` 中第二小的值是 `4` 。

给你 `n`、`edges`、`time` 和 `change` ，返回从节点 `1` 到节点 `n` 需要的 **第二短时间** 。

#### 注意：
* 你可以 **任意次** 穿过任意顶点，**包括** `1` 和 `n` 。
* 你可以假设在 **启程时** ，所有信号灯刚刚变成 **绿色** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/09/29/e2.png)
<pre>
<strong>输入:</strong> n = 5, edges = [[1,2],[1,3],[1,4],[3,4],[4,5]], time = 3, change = 5
<strong>输出:</strong> 13
<strong>解释:</strong>
上面的左图展现了给出的城市交通图。
右图中的蓝色路径是最短时间路径。
花费的时间是：
- 从节点 1 开始，总花费时间=0
- 1 -> 4：3 分钟，总花费时间=3
- 4 -> 5：3 分钟，总花费时间=6
因此需要的最小时间是 6 分钟。

右图中的红色路径是第二短时间路径。
- 从节点 1 开始，总花费时间=0
- 1 -> 3：3 分钟，总花费时间=3
- 3 -> 4：3 分钟，总花费时间=6
- 在节点 4 等待 4 分钟，总花费时间=10
- 4 -> 5：3 分钟，总花费时间=13
因此第二短时间是 13 分钟。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/09/29/eg2.png)
<pre>
<strong>输入:</strong> n = 2, edges = [[1,2]], time = 3, change = 2
<strong>输出:</strong> 11
<strong>解释:</strong>
最短时间路径是 1 -> 2 ，总花费时间 = 3 分钟
第二短时间路径是 1 -> 2 -> 1 -> 2 ，总花费时间 = 11 分钟
</pre>

#### 提示:
* <code>2 <= n <= 10<sup>4</sup></code>
* <code>n - 1 <= edges.length <= min(2 * 10<sup>4</sup>, n * (n - 1) / 2)</code>
* `edges[i].length == 2`
* <code>1 <= u<sub>i</sub>, v<sub>i</sub> <= n</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* 不含重复边
* 每个节点都可以从其他节点直接或者间接到达
* <code>1 <= time, change <= 10<sup>3</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        use std::collections::HashMap;
        use std::collections::VecDeque;

        let n = n as usize;
        let mut neighbors = vec![vec![]; n + 1];
        let mut visited = HashMap::from([(1, 0)]);
        let mut deque = VecDeque::from([(1, 0)]);
        let mut min_step = i32::MAX;
        let mut smin_step = i32::MAX;
        let mut ret = 0;

        for edge in &edges {
            neighbors[edge[0] as usize].push(edge[1] as usize);
            neighbors[edge[1] as usize].push(edge[0] as usize);
        }

        while let Some((u, step)) = deque.pop_front() {
            if u == n {
                if step < min_step {
                    min_step = step;
                    smin_step = step + 2;
                } else if step > min_step {
                    smin_step = step;
                }
            }

            if step >= smin_step {
                break;
            }

            for &v in &neighbors[u] {
                if !visited.contains_key(&v) {
                    visited.insert(v, step + 1);
                    deque.push_back((v, step + 1));
                } else if visited[&v] == step {
                    visited.insert(v, -1);
                    deque.push_back((v, step + 1));
                }
            }
        }

        for _ in 0..smin_step {
            if ret % (2 * change) >= change {
                ret = (ret / (2 * change) + 1) * 2 * change;
            }

            ret += time;
        }

        ret
    }
}
```
