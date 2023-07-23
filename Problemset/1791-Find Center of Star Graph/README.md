# 1791. Find Center of Star Graph
There is an undirected **star** graph consisting of `n` nodes labeled from `1` to `n`. A star graph is a graph where there is one **center** node and **exactly** `n - 1` edges that connect the center node with every other node.

You are given a 2D integer array `edges` where each <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> indicates that there is an edge between the nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>. Return the center of the given star graph.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/02/24/star_graph.png)
<pre>
<strong>Input:</strong> edges = [[1,2],[2,3],[4,2]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> As shown in the figure above, node 2 is connected to every other node, so 2 is the center.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> edges = [[1,2],[5,1],[1,3],[1,4]]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* <code>3 <= n <= 10<sup>5</sup></code>
* `edges.length == n - 1`
* `edges[i].length == 2`
* <code>1 <= u<sub>i</sub>, v<sub>i</sub> <= n</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* The given `edges` represent a valid star graph.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[][]} edges
# @return {Integer}
def find_center(edges)
  edges[0].include?(edges[1][0]) ? edges[1][0] : edges[1][1]
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if edges[0].contains(&edges[1][0]) {
            edges[1][0]
        } else {
            edges[1][1]
        }
    }
}
```
