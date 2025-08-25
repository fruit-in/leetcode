# 2421. Number of Good Paths
There is a tree (i.e. a connected, undirected graph with no cycles) consisting of `n` nodes numbered from `0` to `n - 1` and exactly `n - 1` edges.

You are given a **0-indexed** integer array `vals` of length `n` where `vals[i]` denotes the value of the <code>i<sup>th</sup></code> node. You are also given a 2D integer array `edges` where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> denotes that there exists an **undirected** edge connecting nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code>.

A **good path** is a simple path that satisfies the following conditions:
1. The starting node and the ending node have the **same** value.
2. All nodes between the starting node and the ending node have values **less than or equal to** the starting node (i.e. the starting node's value should be the maximum value along the path).

Return *the number of distinct good paths*.

Note that a path and its reverse are counted as the **same** path. For example, `0 -> 1` is considered to be the same as `1 -> 0`. A single node is also considered as a valid path.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/08/04/f9caaac15b383af9115c5586779dec5.png)
<pre>
<strong>Input:</strong> vals = [1,3,2,1,3], edges = [[0,1],[0,2],[2,3],[2,4]]
<strong>Output:</strong> 6
<strong>Explanation:</strong> There are 5 good paths consisting of a single node.
There is 1 additional good path: 1 -> 0 -> 2 -> 4.
(The reverse path 4 -> 2 -> 0 -> 1 is treated as the same as 1 -> 0 -> 2 -> 4.)
Note that 0 -> 2 -> 3 is not a good path because vals[2] > vals[0].
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/08/04/149d3065ec165a71a1b9aec890776ff.png)
<pre>
<strong>Input:</strong> vals = [1,1,2,2,3], edges = [[0,1],[1,2],[2,3],[2,4]]
<strong>Output:</strong> 7
<strong>Explanation:</strong> There are 5 good paths consisting of a single node.
There are 2 additional good paths: 0 -> 1 and 2 -> 3.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2022/08/04/31705e22af3d9c0a557459bc7d1b62d.png)
<pre>
<strong>Input:</strong> vals = [1], edges = []
<strong>Output:</strong> 1
<strong>Explanation:</strong> The tree consists of only one node, so there is one good path.
</pre>

#### Constraints:
* `n == vals.length`
* <code>1 <= n <= 3 * 10<sup>4</sup></code>
* <code>0 <= vals[i] <= 10<sup>5</sup></code>
* `edges.length == n - 1`
* `edges[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* `edges` represents a valid tree.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def numberOfGoodPaths(self, vals: List[int], edges: List[List[int]]) -> int:
        n = len(vals)
        parent = list(range(n))
        neighbors = [[] for _ in range(n)]
        valscount = Counter(vals)
        ret = sum(1 for c in valscount.values() if c == 1)

        for a, b in edges:
            neighbors[a].append(b)
            neighbors[b].append(a)

        for i in sorted(range(n), key=lambda i: vals[i]):
            if parent[i] != i or valscount[vals[i]] == 1:
                continue

            sameval = 1
            visited = {i}
            greater = set()

            while neighbors[i]:
                j = neighbors[i].pop()
                while parent[j] != parent[parent[j]]:
                    parent[j] = parent[parent[j]]
                j = parent[j]

                if vals[j] == vals[i]:
                    sameval += 1
                if vals[j] <= vals[i] and parent[j] != i:
                    parent[j] = i
                    for k in neighbors[j]:
                        while parent[k] != parent[parent[k]]:
                            parent[k] = parent[parent[k]]
                        k = parent[k]
                        if k not in visited:
                            neighbors[i].append(k)
                            visited.add(k)
                elif vals[j] > vals[i]:
                    greater.add(j)

            neighbors[i] = list(greater)
            ret += sameval * (sameval + 1) // 2

        return ret
```
