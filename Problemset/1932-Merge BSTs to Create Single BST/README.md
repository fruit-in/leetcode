# 1932. Merge BSTs to Create Single BST
You are given `n` **BST (binary search tree) root nodes** for `n` separate BSTs stored in an array `trees` (**0-indexed**). Each BST in `trees` has **at most 3 nodes**, and no two roots have the same value. In one operation, you can:
* Select two **distinct** indices `i` and `j` such that the value stored at one of the **leaves** of `trees[i]` is equal to the **root value** of `trees[j]`.
* Replace the leaf node in `trees[i]` with `trees[j]`.
* Remove `trees[j]` from `trees`.

Return *the **root** of the resulting BST if it is possible to form a valid BST after performing* `n - 1` *operations, or* `null` *if it is impossible to create a valid BST*.

A BST (binary search tree) is a binary tree where each node satisfies the following property:
* Every node in the node's left subtree has a value **strictly less** than the node's value.
* Every node in the node's right subtree has a value **strictly greater** than the node's value.

A leaf is a node that has no children.

#### Example 1:
<pre>
<img src="https://assets.leetcode.com/uploads/2021/06/08/d1.png">
<strong>Input:</strong> trees = [[2,1],[3,2,5],[5,4]]
<strong>Output:</strong> [3,2,5,1,null,4]
<strong>Explanation:</strong>
In the first operation, pick i=1 and j=0, and merge trees[0] into trees[1].
Delete trees[0], so trees = [[3,2,5,1],[5,4]].
<img src="https://assets.leetcode.com/uploads/2021/06/24/diagram.png">
In the second operation, pick i=0 and j=1, and merge trees[1] into trees[0].
Delete trees[1], so trees = [[3,2,5,1,null,4]].
<img src="https://assets.leetcode.com/uploads/2021/06/24/diagram-2.png">
The resulting tree, shown above, is a valid BST, so return its root.
</pre>

#### Example 2:
<pre>
<img src="https://assets.leetcode.com/uploads/2021/06/08/d2.png">
<strong>Input:</strong> trees = [[5,3,8],[3,2,6]]
<strong>Output:</strong> []
<strong>Explanation:</strong>
Pick i=0 and j=1 and merge trees[1] into trees[0].
Delete trees[1], so trees = [[5,3,8,2,6]].
<img src="https://assets.leetcode.com/uploads/2021/06/24/diagram-3.png">
The resulting tree is shown above. This is the only valid operation that can be performed, but the resulting tree is not a valid BST, so return null.
</pre>

#### Example 3:
<pre>
<img src="https://assets.leetcode.com/uploads/2021/06/08/d3.png">
<strong>Input:</strong> trees = [[5,4],[3]]
<strong>Output:</strong> []
<strong>Explanation:</strong> It is impossible to perform any operations.
</pre>

#### Constraints:
* `n == trees.length`
* <code>1 <= n <= 5 * 10<sup>4</sup></code>
* The number of nodes in each tree is in the range `[1, 3]`.
* Each node in the input may have children but no grandchildren.
* No two roots of `trees` have the same value.
* All the trees in the input are **valid BSTs**.
* <code>1 <= TreeNode.val <= 5 * 10<sup>4</sup></code>.

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
    def canMerge(self, trees: List[TreeNode]) -> Optional[TreeNode]:
        def buildTree(root: TreeNode) -> None:
            if root.left and root.left.val in trees:
                root.left = trees.pop(root.left.val)
                buildTree(root.left)
            if root.right and root.right.val in trees:
                root.right = trees.pop(root.right.val)
                buildTree(root.right)

        def isValid(root: TreeNode) -> (bool, int, int):
            minval, maxval = root.val, root.val
            if root.left:
                leftvalid, leftmin, leftmax = isValid(root.left)
                if not leftvalid or leftmax >= root.val:
                    return (False, minval, maxval)
                minval = leftmin
            if root.right:
                rightvalid, rightmin, rightmax = isValid(root.right)
                if not rightvalid or rightmin <= root.val:
                    return (False, minval, maxval)
                maxval = rightmax

            return (True, minval, maxval)

        trees = {tree.val: tree for tree in trees}
        leaves = set()

        for tree in trees.values():
            if tree.left:
                if tree.left.val in leaves:
                    return None
                leaves.add(tree.left.val)
            if tree.right:
                if tree.right.val in leaves:
                    return None
                leaves.add(tree.right.val)

        candidates = {tree for tree in trees if tree not in leaves}

        if len(candidates) != 1:
            return None

        root = trees.pop(candidates.pop())
        buildTree(root)

        if trees or not isValid(root)[0]:
            return None

        return root
```
