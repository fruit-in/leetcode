# 1617. Count Subtrees With Max Distance Between Cities
There are `n` cities numbered from `1` to `n`. You are given an array `edges` of size `n-1`, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> represents a bidirectional edge between cities <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>. There exists a unique path between each pair of cities. In other words, the cities form a **tree**.

A **subtree** is a subset of cities where every city is reachable from every other city in the subset, where the path between each pair passes through only the cities from the subset. Two subtrees are different if there is a city in one subtree that is not present in the other.

For each `d` from `1` to `n-1`, find the number of subtrees in which the **maximum distance** between any two cities in the subtree is equal to `d`.

Return *an array of size* `n-1` *where the* <code>d<sup>th</sup></code> *element **(1-indexed)** is the number of subtrees in which the **maximum distance** between any two cities is equal to* `d`.

**Notice** that the **distance** between the two cities is the number of edges in the path between them.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/21/p1.png)
<pre>
<strong>Input:</strong> n = 4, edges = [[1,2],[2,3],[2,4]]
<strong>Output:</strong> [3,4,0]
<strong>Explanation:</strong>
The subtrees with subsets {1,2}, {2,3} and {2,4} have a max distance of 1.
The subtrees with subsets {1,2,3}, {1,2,4}, {2,3,4} and {1,2,3,4} have a max distance of 2.
No subtree has two nodes where the max distance between them is 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2, edges = [[1,2]]
<strong>Output:</strong> [1]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 3, edges = [[1,2],[2,3]]
<strong>Output:</strong> [2,1]
</pre>

#### Constraints:
* `2 <= n <= 15`
* `edges.length == n-1`
* `edges[i].length == 2`
* <code>1 <= u<sub>i</sub>, v<sub>i</sub> <= n</code>
* All pairs <code>(u<sub>i</sub>, v<sub>i</sub>)</code> are distinct.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countSubgraphsForEachDiameter(self, n: int, edges: List[List[int]]) -> List[int]:
        def buildTree(x: int) -> (int, List[List[int]]):
            countnodes = x.bit_count()
            countedges = 0
            root = -1
            children = [[] for _ in range(n)]

            for u, v in edges:
                if (x >> u) & 1 == 1 and (x >> v) & 1 == 1:
                    countedges += 1
                    root = u
                    children[u].append(v)
                    children[v].append(u)

            if countnodes == 1 or countnodes - countedges != 1:
                return (-1, [])

            stack = [root]
            while stack:
                node = stack.pop()
                for child in children[node]:
                    children[child].remove(node)
                    stack.append(child)

            return (root, children)

        def maxDistance(root: int, children: List[List[int]]) -> (int, int):
            maxdist, maxheight = 0, 0
            c0, h0, c1, h1 = 0, -1, 0, -1

            for child in children[root]:
                dist, height = maxDistance(child, children)
                maxdist = max(maxdist, dist)
                maxheight = max(maxheight, height + 1)

                if height >= h0:
                    c0, h0, c1, h1 = child, height, c0, h0
                elif height >= h1:
                    c1, h1 = child, height

            return (max(maxdist, h0 + h1 + 2), maxheight)

        edges = [[u - 1, v - 1] for u, v in edges]
        ans = [0] * (n - 1)

        for x in range(3, 1 << n):
            root, children = buildTree(x)
            if root >= 0:
                ans[maxDistance(root, children)[0] - 1] += 1

        return ans
```
