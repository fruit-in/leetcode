# 2065. Maximum Path Quality of a Graph
There is an **undirected** graph with `n` nodes numbered from `0` to `n - 1` (**inclusive**). You are given a **0-indexed** integer array `values` where `values[i]` is the **value** of the <code>i<sup>th</sup></code> node. You are also given a **0-indexed** 2D integer array `edges`, where each <code>edges[j] = [u<sub>j</sub>, v<sub>j</sub>, time<sub>j</sub>]</code> indicates that there is an undirected edge between the nodes <code>u<sub>j</sub></code> and <code>v<sub>j</sub></code>, and it takes <code>time<sub>j</sub></code> seconds to travel between the two nodes. Finally, you are given an integer `maxTime`.

A **valid path** in the graph is any path that starts at node `0`, ends at node `0`, and takes **at most** `maxTime` seconds to complete. You may visit the same node multiple times. The **quality** of a valid path is the **sum** of the values of the **unique nodes** visited in the path (each node's value is added **at most once** to the sum).

Return *the **maximum** quality of a valid path*.

**Note:** There are **at most four** edges connected to each node.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/10/19/ex1drawio.png)
<pre>
<strong>Input:</strong> values = [0,32,10,43], edges = [[0,1,10],[1,2,15],[0,3,10]], maxTime = 49
<strong>Output:</strong> 75
<strong>Explanation:</strong>
One possible path is 0 -> 1 -> 0 -> 3 -> 0. The total time taken is 10 + 10 + 10 + 10 = 40 <= 49.
The nodes visited are 0, 1, and 3, giving a maximal path quality of 0 + 32 + 43 = 75.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/10/19/ex2drawio.png)
<pre>
<strong>Input:</strong> values = [5,10,15,20], edges = [[0,1,10],[1,2,10],[0,3,10]], maxTime = 30
<strong>Output:</strong> 25
<strong>Explanation:</strong>
One possible path is 0 -> 3 -> 0. The total time taken is 10 + 10 = 20 <= 30.
The nodes visited are 0 and 3, giving a maximal path quality of 5 + 20 = 25.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/10/19/ex31drawio.png)
<pre>
<strong>Input:</strong> values = [1,2,3,4], edges = [[0,1,10],[1,2,11],[2,3,12],[1,3,13]], maxTime = 50
<strong>Output:</strong> 7
<strong>Explanation:</strong> One possible path is 0 -> 1 -> 3 -> 1 -> 0. The total time taken is 10 + 13 + 13 + 10 = 46 <= 50.
The nodes visited are 0, 1, and 3, giving a maximal path quality of 1 + 2 + 4 = 7.
</pre>

#### Constraints:
* `n == values.length`
* `1 <= n <= 1000`
* <code>0 <= values[i] <= 10<sup>8</sup></code>
* `0 <= edges.length <= 2000`
* `edges[j].length == 3 `
* <code>0 <= u<sub>j</sub> < v<sub>j</sub> <= n - 1</code>
* <code>10 <= time<sub>j</sub>, maxTime <= 100</code>
* All the pairs <code>[u<sub>j</sub>, v<sub>j</sub>]</code> are **unique**.
* There are **at most four** edges connected to each node.
* The graph may not be connected.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def maximalPathQuality(self, values: List[int], edges: List[List[int]], maxTime: int) -> int:
        def dfs(u: int, time: int) -> None:
            nonlocal quality, ret
            if u == 0:
                ret = max(ret, quality)
            if usetimes[u] == 0:
                quality += values[u]
            usetimes[u] += 1

            for v, t in neighbors[u].items():
                if time + t <= maxTime:
                    dfs(v, time + t)

            usetimes[u] -= 1
            if usetimes[u] == 0:
                quality -= values[u]

        n = len(values)
        neighbors = [{} for _ in range(n)]
        usetimes = [0] * n
        quality = 0
        ret = values[0]

        for u, v, time in edges:
            if time <= maxTime:
                neighbors[u][v] = time
                neighbors[v][u] = time

        dfs(0, 0)

        return ret
```
