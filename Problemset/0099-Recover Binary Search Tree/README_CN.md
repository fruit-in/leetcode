# 99. 恢复二叉搜索树
给你二叉搜索树的根节点 `root` ，该树中的 **恰好** 两个节点的值被错误地交换。*请在不改变其结构的情况下，恢复这棵树 *。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/10/28/recover1.jpg)
<pre>
<strong>输入:</strong> root = [1,3,null,null,2]
<strong>输出:</strong> [3,1,null,null,2]
<strong>解释:</strong> 3 不能是 1 的左孩子，因为 3 > 1 。交换 1 和 3 使二叉搜索树有效。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/10/28/recover2.jpg)
<pre>
<strong>输入:</strong> root = [3,1,4,null,null,2]
<strong>输出:</strong> [2,1,4,null,null,3]
<strong>解释:</strong> 2 不能在 3 的右子树中，因为 2 < 3 。交换 2 和 3 使二叉搜索树有效。
</pre>

#### 提示:
* 树上节点的数目在范围 `[2, 1000]` 内
* <code>-2<sup>31</sup> <= Node.val <= 2<sup>31</sup> - 1</code>

**进阶：**使用 `O(n)` 空间复杂度的解法很容易实现。你能想出一个只使用 `O(1)` 空间的解决方案吗？

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
    def recoverTree(self, root: Optional[TreeNode]) -> None:
        """
        Do not return anything, modify root in-place instead.
        """
        curr = root
        node1, node2 = None, TreeNode(-inf)
        wrong1, wrong2 = None, None

        while curr:
            if not curr.left:
                node1, node2 = node2, curr
                if not wrong1 and node1.val > node2.val:
                    wrong1 = node1
                if node1.val > node2.val:
                    wrong2 = node2
                curr = curr.right
            else:
                temp = curr.left
                while temp.right and temp.right != curr:
                    temp = temp.right
                if not temp.right:
                    temp.right = curr
                    curr = curr.left
                else:
                    node1, node2 = node2, curr
                    if not wrong1 and node1.val > node2.val:
                        wrong1 = node1
                    if node1.val > node2.val:
                        wrong2 = node2
                    temp.right = None
                    curr = curr.right

        wrong1.val, wrong2.val = wrong2.val, wrong1.val
```
