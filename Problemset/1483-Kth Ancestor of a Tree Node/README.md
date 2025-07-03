# 1483. Kth Ancestor of a Tree Node
You are given a tree with `n` nodes numbered from `0` to `n - 1` in the form of a parent array `parent` where `parent[i]` is the parent of <code>i<sup>th</sup></code> node. The root of the tree is node `0`. Find the <code>k<sup>th</sup></code> ancestor of a given node.

The <code>k<sup>th</sup></code> ancestor of a tree node is the <code>k<sup>th</sup></code> node in the path from that node to the root node.

Implement the `TreeAncestor` class:
* `TreeAncestor(int n, int[] parent)` Initializes the object with the number of nodes in the tree and the parent array.
* `int getKthAncestor(int node, int k)` return the <code>k<sup>th</sup></code> ancestor of the given node `node`. If there is no such ancestor, return `-1`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/08/28/1528_ex1.png)
<pre>
<strong>Input:</strong>
["TreeAncestor", "getKthAncestor", "getKthAncestor", "getKthAncestor"]
[[7, [-1, 0, 0, 1, 1, 2, 2]], [3, 1], [5, 2], [6, 3]]
<strong>Output:</strong>
[null, 1, 0, -1]
<strong>Explanation:</strong>
TreeAncestor treeAncestor = new TreeAncestor(7, [-1, 0, 0, 1, 1, 2, 2]);
treeAncestor.getKthAncestor(3, 1); // returns 1 which is the parent of 3
treeAncestor.getKthAncestor(5, 2); // returns 0 which is the grandparent of 5
treeAncestor.getKthAncestor(6, 3); // returns -1 because there is no such ancestor
</pre>

#### Constraints:
* <code>1 <= k <= n <= 5 * 10<sup>4</sup></code>
* `parent.length == n`
* `parent[0] == -1`
* `0 <= parent[i] < n` for all `0 < i < n`
* `0 <= node < n`
* There will be at most <code>5 * 10<sup>4</sup></code> queries.

## Solutions (Python)

### 1. Solution
```Python
class TreeAncestor:

    def __init__(self, n: int, parent: List[int]):
        def dfs(i: int):
            while 1 << len(self.ancestors[i]) <= self.depth[i]:
                self.ancestors[i].append(self.getKthAncestor(
                    parent[i], ((1 << len(self.ancestors[i])) - 1)))
            for child in children[i]:
                self.depth[child] = self.depth[i] + 1
                dfs(child)

        children = [[] for _ in range(n)]
        self.depth = [0] * n
        self.ancestors = [[] for _ in range(n)]
        for i in range(1, len(parent)):
            children[parent[i]].append(i)
        dfs(0)

    def getKthAncestor(self, node: int, k: int) -> int:
        if k == 0:
            return node
        if k > self.depth[node]:
            return -1

        return self.getKthAncestor(self.ancestors[node][k.bit_length() - 1], k ^ (1 << (k.bit_length() - 1)))


# Your TreeAncestor object will be instantiated and called as such:
# obj = TreeAncestor(n, parent)
# param_1 = obj.getKthAncestor(node,k)
```
