# 979. Distribute Coins in Binary Tree
You are given the `root` of a binary tree with `n` nodes where each `node` in the tree has `node.val` coins. There are `n` coins in total throughout the whole tree.

In one move, we may choose two adjacent nodes and move one coin from one node to another. A move may be from parent to child, or from child to parent.

Return *the **minimum** number of moves required to make every node have **exactly** one coin*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/01/18/tree1.png)
<pre>
<strong>Input:</strong> root = [3,0,0]
<strong>Output:</strong> 2
<strong>Explanation:</strong> From the root of the tree, we move one coin to its left child, and one coin to its right child.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/01/18/tree2.png)
<pre>
<strong>Input:</strong> root = [0,3,0]
<strong>Output:</strong> 3
<strong>Explanation:</strong> From the left child of the root, we move two coins to the root [taking two moves]. Then, we move one coin from the root of the tree to the right child.
</pre>

#### Constraints:
* The number of nodes in the tree is `n`.
* `1 <= n <= 100`
* `0 <= Node.val <= n`
* The sum of all `Node.val` is `n`.

## Solutions (Python)

### 1. Solution
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def distributeCoins(self, root: Optional[TreeNode]) -> int:
        return self.dfs(root)[2]

    def dfs(self, root: Optional[TreeNode]) -> (int, int, int):
        if root is None:
            return (0, 0, 0)

        lnodes, lvals, lmoves = self.dfs(root.left)
        rnodes, rvals, rmoves = self.dfs(root.right)

        return (lnodes + rnodes + 1,
                lvals + rvals + root.val,
                lmoves + rmoves + abs(lnodes - lvals) + abs(rnodes - rvals))
```
