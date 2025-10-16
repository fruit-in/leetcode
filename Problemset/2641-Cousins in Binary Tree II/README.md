# 2641. Cousins in Binary Tree II
Given the `root` of a binary tree, replace the value of each node in the tree with the **sum of all its cousins' values**.

Two nodes of a binary tree are **cousins** if they have the same depth with different parents.

Return *the* `root` *of the modified tree*.

**Note** that the depth of a node is the number of edges in the path from the root node to it.

#### Example 1:
![](https://assets.leetcode.com/uploads/2023/01/11/example11.png)
<pre>
<strong>Input:</strong> root = [5,4,9,1,10,null,7]
<strong>Output:</strong> [0,0,0,7,7,null,11]
<strong>Explanation:</strong> The diagram above shows the initial binary tree and the binary tree after changing the value of each node.
- Node with value 5 does not have any cousins so its sum is 0.
- Node with value 4 does not have any cousins so its sum is 0.
- Node with value 9 does not have any cousins so its sum is 0.
- Node with value 1 has a cousin with value 7 so its sum is 7.
- Node with value 10 has a cousin with value 7 so its sum is 7.
- Node with value 7 has cousins with values 1 and 10 so its sum is 11.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2023/01/11/diagram33.png)
<pre>
<strong>Input:</strong> root = [3,1,2]
<strong>Output:</strong> [0,0,0]
<strong>Explanation:</strong> The diagram above shows the initial binary tree and the binary tree after changing the value of each node.
- Node with value 3 does not have any cousins so its sum is 0.
- Node with value 1 does not have any cousins so its sum is 0.
- Node with value 2 does not have any cousins so its sum is 0.
</pre>

#### Constraints:
* The number of nodes in the tree is in the range <code>[1, 10<sup>5</sup>]</code>.
* <code>1 <= Node.val <= 10<sup>4</sup></code>

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
    def replaceValueInTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        def dfsLevelSums(root: Optional[TreeNode], depth: int) -> None:
            if not root:
                return

            while depth >= len(levelsums):
                levelsums.append(0)
            levelsums[depth] += root.val

            dfsLevelSums(root.left, depth + 1)
            dfsLevelSums(root.right, depth + 1)

        def dfsCousins(root: Optional[TreeNode], depth: int) -> None:
            if not root:
                return

            leftval = root.left.val if root.left else 0
            rightval = root.right.val if root.right else 0

            if root.left:
                root.left.val = levelsums[depth + 1] - leftval - rightval
                dfsCousins(root.left, depth + 1)
            if root.right:
                root.right.val = levelsums[depth + 1] - leftval - rightval
                dfsCousins(root.right, depth + 1)

        levelsums = []
        dfsLevelSums(root, 0)
        dfsCousins(root, 0)
        root.val = 0

        return root
```
