# 2359. 找到离给定两个节点最近的节点
给你一个 `n` 个节点的 **有向图** ，节点编号为 `0` 到 `n - 1` ，每个节点 **至多** 有一条出边。

有向图用大小为 `n` 下标从 **0** 开始的数组 `edges` 表示，表示节点 i 有一条有向边指向 `edges[i]` 。如果节点 `i` 没有出边，那么 `edges[i] == -1` 。

同时给你两个节点 `node1` 和 `node2` 。

请你返回一个从 `node1` 和 `node2` 都能到达节点的编号，使节点 `node1` 和节点 `node2` 到这个节点的距离 **较大值最小化**。如果有多个答案，请返回 **最小** 的节点编号。如果答案不存在，返回 `-1` 。

注意 `edges` 可能包含环。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/06/07/graph4drawio-2.png)
<pre>
<strong>输入:</strong> edges = [2,2,3,-1], node1 = 0, node2 = 1
<strong>输出:</strong> 2
<strong>解释:</strong> 从节点 0 到节点 2 的距离为 1 ，从节点 1 到节点 2 的距离为 1 。
两个距离的较大值为 1 。我们无法得到一个比 1 更小的较大值，所以我们返回节点 2 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/06/07/graph4drawio-4.png)
<pre>
<strong>输入:</strong> edges = [1,2,-1], node1 = 0, node2 = 2
<strong>输出:</strong> 2
<strong>解释:</strong> 节点 0 到节点 2 的距离为 2 ，节点 2 到它自己的距离为 0 。
两个距离的较大值为 2 。我们无法得到一个比 2 更小的较大值，所以我们返回节点 2 。
</pre>

#### 提示:
* `n == edges.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `-1 <= edges[i] < n`
* `edges[i] != i`
* `0 <= node1, node2 < n`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();
        let mut distances1 = vec![-1; n];
        let mut distances2 = vec![-1; n];
        let mut i = node1 as usize;
        let mut d = 0;
        let mut min_d = i32::MAX;
        let mut ret = -1;

        while distances1[i] == -1 {
            distances1[i] = d;
            if edges[i] == -1 {
                break;
            }
            i = edges[i] as usize;
            d += 1;
        }

        i = node2 as usize;
        d = 0;

        while distances2[i] == -1 {
            distances2[i] = d;
            if edges[i] == -1 {
                break;
            }
            i = edges[i] as usize;
            d += 1;
        }

        for i in 0..n {
            if distances1[i] != -1 && distances2[i] != -1 {
                let d = distances1[i].max(distances2[i]);

                if ret == -1 || d < min_d {
                    min_d = d;
                    ret = i as i32;
                }
            }
        }

        ret
    }
}
```
