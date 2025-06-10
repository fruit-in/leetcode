# 2538. Difference Between Maximum and Minimum Price Sum
There exists an undirected and initially unrooted tree with `n` nodes indexed from `0` to `n - 1`. You are given the integer `n` and a 2D integer array `edges` of length `n - 1`, where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> in the tree.

Each node has an associated price. You are given an integer array `price`, where `price[i]` is the price of the <code>i<sup>th</sup></code> node.

The **price sum** of a given path is the sum of the prices of all nodes lying on that path.

The tree can be rooted at any node `root` of your choice. The incurred **cost** after choosing `root` is the difference between the maximum and minimum **price sum** amongst all paths starting at `root`.

Return *the **maximum** possible **cost** amongst all possible root choices*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/12/01/example14.png)
<pre>
<strong>Input:</strong> n = 6, edges = [[0,1],[1,2],[1,3],[3,4],[3,5]], price = [9,8,7,6,10,5]
<strong>Output:</strong> 24
<strong>Explanation:</strong> The diagram above denotes the tree after rooting it at node 2. The first part (colored in red) shows the path with the maximum price sum. The second part (colored in blue) shows the path with the minimum price sum.
- The first path contains nodes [2,1,3,4]: the prices are [7,8,6,10], and the sum of the prices is 31.
- The second path contains the node [2] with the price [7].
The difference between the maximum and minimum price sum is 24. It can be proved that 24 is the maximum cost.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/11/24/p1_example2.png)
<pre>
<strong>Input:</strong> n = 3, edges = [[0,1],[1,2]], price = [1,1,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The diagram above denotes the tree after rooting it at node 0. The first part (colored in red) shows the path with the maximum price sum. The second part (colored in blue) shows the path with the minimum price sum.
- The first path contains nodes [0,1,2]: the prices are [1,1,1], and the sum of the prices is 3.
- The second path contains node [0] with a price [1].
The difference between the maximum and minimum price sum is 2. It can be proved that 2 is the maximum cost.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>
* `edges.length == n - 1`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1</code>
* `edges` represents a valid tree.
* `price.length == n`
* <code>1 <= price[i] <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def maxOutput(self, n: int, edges: List[List[int]], price: List[int]) -> int:
        def removeParentFromChildren(node: int, parent: int) -> None:
            i = children[node].index(parent)
            children[node][i], children[node][-1] = children[node][-1], children[node][i]
            children[node].pop()
            for child in children[node]:
                removeParentFromChildren(child, node)

        def setTop2(node: int) -> None:
            a, c0, b, c1 = -1, 0, -1, 0
            for child in children[node]:
                setTop2(child)
                c2 = top2[child][1] + price[child]
                if c2 >= c0:
                    a, c0, b, c1 = child, c2, a, c0
                elif c2 > c1:
                    b, c1 = child, c2
            top2[node] = [a, c0, b, c1]

        def maxCost(node: int, p: int) -> int:
            a, c0, _, c1 = top2[node]
            cost = max(c0, p)
            for child in children[node]:
                if child != a:
                    cost = max(cost, maxCost(child, max(p, c0) + price[node]))
                else:
                    cost = max(cost, maxCost(child, max(p, c1) + price[node]))
            return cost

        children = [[] for _ in range(n)]
        children[0].append(-1)
        top2 = [[] for _ in range(n)]

        for a, b in edges:
            children[a].append(b)
            children[b].append(a)

        removeParentFromChildren(0, -1)
        setTop2(0)

        return maxCost(0, 0)
```
