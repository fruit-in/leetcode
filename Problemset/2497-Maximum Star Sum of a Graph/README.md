# 2497. Maximum Star Sum of a Graph
There is an undirected graph consisting of `n` nodes numbered from `0` to `n - 1`. You are given a **0-indexed** integer array `vals` of length `n` where `vals[i]` denotes the value of the <code>i<sup>th</sup></code> node.

You are also given a 2D integer array `edges` where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> denotes that there exists an **undirected** edge connecting nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code>.

A **star graph** is a subgraph of the given graph having a center node containing `0` or more neighbors. In other words, it is a subset of edges of the given graph such that there exists a common node for all edges.

The image below shows star graphs with 3 and 4 neighbors respectively, centered at the blue node.

![](https://assets.leetcode.com/uploads/2022/11/07/max-star-sum-descdrawio.png)

The **star sum** is the sum of the values of all the nodes present in the star graph.

Given an integer `k`, return *the **maximum star sum** of a star graph containing **at most*** `k` *edges*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/11/07/max-star-sum-example1drawio.png)
<pre>
<strong>Input:</strong> vals = [1,2,3,4,10,-10,-20], edges = [[0,1],[1,2],[1,3],[3,4],[3,5],[3,6]], k = 2
<strong>Output:</strong> 16
<strong>Explanation:</strong> The above diagram represents the input graph.
The star graph with the maximum star sum is denoted by blue. It is centered at 3 and includes its neighbors 1 and 4.
It can be shown it is not possible to get a star graph with a sum greater than 16.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> vals = [-5], edges = [], k = 0
<strong>Output:</strong> -5
<strong>Explanation:</strong> There is only one possible star graph, which is node 0 itself.
Hence, we return -5.
</pre>

#### Constraints:
* `n == vals.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= vals[i] <= 10<sup>4</sup></code>
* <code>0 <= edges.length <= min(n * (n - 1) / 2, 10<sup>5</sup>)</code>
* `edges[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* `0 <= k <= n - 1`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut heaps = vec![BinaryHeap::new(); vals.len()];
        let mut star_sum = vals.clone();

        for edge in &edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);

            if vals[b] > 0 {
                heaps[a].push(-vals[b]);
                star_sum[a] += vals[b];
            }
            if vals[a] > 0 {
                heaps[b].push(-vals[a]);
                star_sum[b] += vals[a];
            }
            if heaps[a].len() > k {
                star_sum[a] -= -heaps[a].pop().unwrap();
            }
            if heaps[b].len() > k {
                star_sum[b] -= -heaps[b].pop().unwrap();
            }
        }

        *star_sum.iter().max().unwrap()
    }
}
```
