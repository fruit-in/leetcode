# 2359. Find Closest Node to Given Two Nodes
You are given a **directed** graph of `n` nodes numbered from `0` to `n - 1`, where each node has **at most one** outgoing edge.

The graph is represented with a given **0-indexed** array `edges` of size `n`, indicating that there is a directed edge from node `i` to node `edges[i]`. If there is no outgoing edge from `i`, then `edges[i] == -1`.

You are also given two integers `node1` and `node2`.

Return *the **index** of the node that can be reached from both* `node1` *and* `node2`, *such that the **maximum** between the distance from* `node1` *to that node, and from* `node2` *to that node is **minimized***. If there are multiple answers, return the node with the **smallest** index, and if no possible answer exists, return `-1`.

Note that `edges` may contain cycles.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/06/07/graph4drawio-2.png)
<pre>
<strong>Input:</strong> edges = [2,2,3,-1], node1 = 0, node2 = 1
<strong>Output:</strong> 2
<strong>Explanation:</strong> The distance from node 0 to node 2 is 1, and the distance from node 1 to node 2 is 1.
The maximum of those two distances is 1. It can be proven that we cannot get a node with a smaller maximum distance than 1, so we return node 2.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/06/07/graph4drawio-4.png)
<pre>
<strong>Input:</strong> edges = [1,2,-1], node1 = 0, node2 = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> The distance from node 0 to node 2 is 2, and the distance from node 2 to itself is 0.
The maximum of those two distances is 2. It can be proven that we cannot get a node with a smaller maximum distance than 2, so we return node 2.
</pre>

#### Constraints:
* `n == edges.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `-1 <= edges[i] < n`
* `edges[i] != i`
* `0 <= node1, node2 < n`

## Solutions (Rust)

### 1. Solution
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
