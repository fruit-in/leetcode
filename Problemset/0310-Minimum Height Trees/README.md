# 310. Minimum Height Trees
A tree is an undirected graph in which any two vertices are connected by *exactly* one path. In other words, any connected graph without simple cycles is a tree.

Given a tree of `n` nodes labelled from `0` to `n - 1`, and an array of `n - 1` `edges` where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an undirected edge between the two nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> in the tree, you can choose any node of the tree as the root. When you select a node `x` as the root, the result tree has height `h`. Among all possible rooted trees, those with minimum height (i.e. `min(h)`)  are called **minimum height trees** (MHTs).

Return *a list of all **MHTs'** root labels*. You can return the answer in **any order**.

The **height** of a rooted tree is the number of edges on the longest downward path between the root and a leaf.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/01/e1.jpg)
<pre>
<strong>Input:</strong> n = 4, edges = [[1,0],[1,2],[1,3]]
<strong>Output:</strong> [1]
<strong>Explanation:</strong> As shown, the height of the tree is 1 when the root is the node with label 1 which is the only MHT.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/09/01/e2.jpg)
<pre>
<strong>Input:</strong> n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]
<strong>Output:</strong> [3,4]
</pre>

#### Constraints:
* <code>1 <= n <= 2 * 10<sup>4</sup></code>
* `edges.length == n - 1`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* All the pairs <code>(a<sub>i</sub>, b<sub>i</sub>)</code> are distinct.
* The given input is **guaranteed** to be a tree and there will be **no repeated** edges.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def findMinHeightTrees(self, n: int, edges: List[List[int]]) -> List[int]:
        def removeParentFromChildren(node: int, parent: int) -> None:
            i = children[node].index(parent)
            children[node][i], children[node][-1] = children[node][-1], children[node][i]
            children[node].pop()
            for child in children[node]:
                removeParentFromChildren(child, node)

        def setHeight(node: int) -> None:
            for child in children[node]:
                setHeight(child)
                heights[node] = max(heights[node], heights[child] + 1)

        def findMinHeight(node: int, p: int) -> None:
            minheights[node] = max(heights[node], p)
            if len(children[node]) > 1:
                top2 = [children[node][0], children[node][1]]
                if heights[top2[0]] < heights[top2[1]]:
                    top2[0], top2[1] = top2[1], top2[0]
                for child in children[node][2:]:
                    if heights[child] >= heights[top2[0]]:
                        top2 = [child, top2[0]]
                    elif heights[child] >= heights[top2[1]]:
                        top2[1] = child
                for child in children[node]:
                    if child != top2[0]:
                        findMinHeight(child, max(p + 1, heights[top2[0]] + 2))
                    else:
                        findMinHeight(child, max(p + 1, heights[top2[1]] + 2))
            elif len(children[node]) == 1:
                findMinHeight(children[node][0], p + 1)

        children = [[] for _ in range(n)]
        children[0].append(-1)
        heights = [0] * n
        minheights = [0] * n

        for a, b in edges:
            children[a].append(b)
            children[b].append(a)

        removeParentFromChildren(0, -1)
        setHeight(0)
        findMinHeight(0, 0)
        minheight = min(minheights)

        return [node for node in range(n) if minheights[node] == minheight]
```
