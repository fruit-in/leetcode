# 1932. 合并多棵二叉搜索树
给你 `n` 个 **二叉搜索树的根节点** ，存储在数组 `trees` 中（**下标从 0 开始**），对应 `n` 棵不同的二叉搜索树。`trees` 中的每棵二叉搜索树 **最多有 3 个节点** ，且不存在值相同的两个根节点。在一步操作中，将会完成下述步骤：
* 选择两个 **不同的** 下标 `i` 和 `j` ，要求满足在 `trees[i]` 中的某个 **叶节点** 的值等于 `trees[j]` 的 **根节点的值** 。
* 用 `trees[j]` 替换 `trees[i]` 中的那个叶节点。
* 从 `trees` 中移除 `trees[j]` 。

如果在执行 `n - 1` 次操作后，能形成一棵有效的二叉搜索树，则返回结果二叉树的 **根节点** ；如果无法构造一棵有效的二叉搜索树，返回 `null` 。

二叉搜索树是一种二叉树，且树中每个节点均满足下述属性：
* 任意节点的左子树中的值都 **严格小于** 此节点的值。
* 任意节点的右子树中的值都 **严格大于** 此节点的值。

叶节点是不含子节点的节点。

#### 示例 1:
<pre>
<img src="https://assets.leetcode.com/uploads/2021/06/08/d1.png">
<strong>输入:</strong> trees = [[2,1],[3,2,5],[5,4]]
<strong>输出:</strong> [3,2,5,1,null,4]
<strong>输出:</strong>
第一步操作中，选出 i=1 和 j=0 ，并将 trees[0] 合并到 trees[1] 中。
删除 trees[0] ，trees = [[3,2,5,1],[5,4]] 。
<img src="https://assets.leetcode.com/uploads/2021/06/24/diagram.png">
在第二步操作中，选出 i=0 和 j=1 ，将 trees[1] 合并到 trees[0] 中。
删除 trees[1] ，trees = [[3,2,5,1,null,4]] 。
<img src="https://assets.leetcode.com/uploads/2021/06/24/diagram-2.png">
结果树如上图所示，为一棵有效的二叉搜索树，所以返回该树的根节点。
</pre>

#### 示例 2:
<pre>
<img src="https://assets.leetcode.com/uploads/2021/06/08/d2.png">
<strong>输入:</strong> trees = [[5,3,8],[3,2,6]]
<strong>输出:</strong> []
<strong>输出:</strong>
选出 i=0 和 j=1 ，然后将 trees[1] 合并到 trees[0] 中。
删除 trees[1] ，trees = [[5,3,8,2,6]] 。
<img src="https://assets.leetcode.com/uploads/2021/06/24/diagram-3.png">
结果树如上图所示。仅能执行一次有效的操作，但结果树不是一棵有效的二叉搜索树，所以返回 null 。
</pre>

#### 示例 3:
<pre>
<img src="https://assets.leetcode.com/uploads/2021/06/08/d3.png">
<strong>输入:</strong> trees = [[5,4],[3]]
<strong>输出:</strong> []
<strong>输出:</strong> 无法执行任何操作。
</pre>

#### 提示:
* `n == trees.length`
* <code>1 <= n <= 5 * 10<sup>4</sup></code>
* 每棵树中节点数目在范围 `[1, 3]` 内。
* 输入数据的每个节点可能有子节点但不存在子节点的子节点
* `trees` 中不存在两棵树根节点值相同的情况。
* 输入中的所有树都是 **有效的二叉树搜索树** 。
* <code>1 <= TreeNode.val <= 5 * 10<sup>4</sup></code>.

## 题解 (Python)

### 1. 题解
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
