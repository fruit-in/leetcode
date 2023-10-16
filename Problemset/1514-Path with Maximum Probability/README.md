# 1514. Path with Maximum Probability
You are given an undirected weighted graph of `n` nodes (0-indexed), represented by an edge list where `edges[i] = [a, b]` is an undirected edge connecting the nodes `a` and `b` with a probability of success of traversing that edge `succProb[i]`.

Given two nodes `start` and `end`, find the path with the maximum probability of success to go from `start` to `end` and return its success probability.

If there is no path from `start` to `end`, **return 0**. Your answer will be accepted if it differs from the correct answer by at most **1e-5**.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/09/20/1558_ex1.png)
<pre>
<strong>Input:</strong> n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.2], start = 0, end = 2
<strong>Output:</strong> 0.25000
<strong>Explanation:</strong> There are two paths from start to end, one having a probability of success = 0.2 and the other has 0.5 * 0.5 = 0.25.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/09/20/1558_ex2.png)
<pre>
<strong>Input:</strong> n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.3], start = 0, end = 2
<strong>Output:</strong> 0.30000
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2019/09/20/1558_ex3.png)
<pre>
<strong>Input:</strong> n = 3, edges = [[0,1]], succProb = [0.5], start = 0, end = 2
<strong>Output:</strong> 0.00000
<strong>Explanation:</strong> There is no path between 0 and 2.
</pre>

#### Constraints:
* `2 <= n <= 10^4`
* `0 <= start, end < n`
* `start != end`
* `0 <= a, b < n`
* `a != b`
* `0 <= succProb.length == edges.length <= 2*10^4`
* `0 <= succProb[i] <= 1`
* There is at most one edge between every two nodes.

## Solutions (Python)

### 1. Solution
```Python
import heapq


class Solution:
    def maxProbability(self, n: int, edges: List[List[int]], succProb: List[float], start_node: int, end_node: int) -> float:
        graph = {start_node: [(1, start_node)]}
        seen = set()
        heap = [(-1, start_node)]
        heapq.heapify(heap)

        for i in range(len(edges)):
            if edges[i][0] not in graph:
                graph[edges[i][0]] = []
            if edges[i][1] not in graph:
                graph[edges[i][1]] = []
            graph[edges[i][0]].append((succProb[i], edges[i][1]))
            graph[edges[i][1]].append((succProb[i], edges[i][0]))

        while len(heap) > 0:
            prob0, node0 = heapq.heappop(heap)
            seen.add(node0)

            if node0 == end_node:
                return -prob0

            for prob1, node1 in graph[node0]:
                if node1 not in seen:
                    heapq.heappush(heap, (prob0 * prob1, node1))

        return 0
```
