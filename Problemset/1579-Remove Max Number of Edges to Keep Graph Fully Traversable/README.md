# 1579. Remove Max Number of Edges to Keep Graph Fully Traversable
Alice and Bob have an undirected graph of `n` nodes and three types of edges:
* Type 1: Can be traversed by Alice only.
* Type 2: Can be traversed by Bob only.
* Type 3: Can be traversed by both Alice and Bob.

Given an array `edges` where <code>edges[i] = [type<sub>i</sub>, u<sub>i</sub>, v<sub>i</sub>]</code> represents a bidirectional edge of type <code>type<sub>i</sub></code> between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>, find the maximum number of edges you can remove so that after removing the edges, the graph can still be fully traversed by both Alice and Bob. The graph is fully traversed by Alice and Bob if starting from any node, they can reach all other nodes.

Return *the maximum number of edges you can remove, or return* `-1` *if Alice and Bob cannot fully traverse the graph*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/08/19/ex1.png)
<pre>
<strong>Input:</strong> n = 4, edges = [[3,1,2],[3,2,3],[1,1,3],[1,2,4],[1,1,2],[2,3,4]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> If we remove the 2 edges [1,1,2] and [1,1,3]. The graph will still be fully traversable by Alice and Bob. Removing any additional edge will not make it so. So the maximum number of edges we can remove is 2.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/08/19/ex2.png)
<pre>
<strong>Input:</strong> n = 4, edges = [[3,1,2],[3,2,3],[1,1,4],[2,1,4]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Notice that removing any edge will not make the graph fully traversable by Alice and Bob.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2020/08/19/ex3.png)
<pre>
<strong>Input:</strong> n = 4, edges = [[3,2,3],[1,1,2],[2,3,4]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> In the current graph, Alice cannot reach node 4 from the other nodes. Likewise, Bob cannot reach 1. Therefore it's impossible to make the graph fully traversable.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= edges.length <= min(10<sup>5</sup>, 3 * n * (n - 1) / 2)</code>
* `edges[i].length == 3`
* <code>1 <= type<sub>i</sub> <= 3</code>
* <code>1 <= u<sub>i</sub> < v<sub>i</sub> <= n</code>
* All tuples <code>(type<sub>i</sub>, u<sub>i</sub>, v<sub>i</sub>)</code> are distinct.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def maxNumEdgesToRemove(self, n: int, edges: List[List[int]]) -> int:
        parenta = list(range(n + 1))
        parentb = list(range(n + 1))
        counta = 0
        countb = 0
        ret = len(edges)

        edges.sort(key=lambda edge: -edge[0])

        for t, u, v in edges:
            while t != 2 and parenta[u] != parenta[parenta[u]]:
                parenta[u] = parenta[parenta[u]]
            while t != 2 and parenta[v] != parenta[parenta[v]]:
                parenta[v] = parenta[parenta[v]]
            while t != 1 and parentb[u] != parentb[parentb[u]]:
                parentb[u] = parentb[parentb[u]]
            while t != 1 and parentb[v] != parentb[parentb[v]]:
                parentb[v] = parentb[parentb[v]]

            if t == 3 and parenta[u] != parenta[v]:
                parenta[parenta[u]] = parenta[v]
                parentb[parentb[u]] = parentb[v]
                counta += 1
                countb += 1
                ret -= 1
            elif t == 2 and parentb[u] != parentb[v]:
                parentb[parentb[u]] = parentb[v]
                countb += 1
                ret -= 1
            elif t == 1 and parenta[u] != parenta[v]:
                parenta[parenta[u]] = parenta[v]
                counta += 1
                ret -= 1

        return ret if counta == countb == n - 1 else -1
```
