# 106. 从中序与后序遍历序列构造二叉树
根据一棵树的中序遍历与后序遍历构造二叉树。

#### 注意:
你可以假设树中没有重复的元素。

例如，给出
```
中序遍历 inorder = [9,3,15,20,7]
后序遍历 postorder = [9,15,7,20,3]
```

返回如下的二叉树：
```
    3
   / \
  9  20
    /  \
   15   7
```

## 题解 (Python)

### 1. 递归
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def buildTree(self, inorder: List[int], postorder: List[int]) -> TreeNode:
        if not postorder:
            return None

        i = inorder.index(postorder[-1])

        return TreeNode(
            postorder[-1],
            self.buildTree(inorder[:i], postorder[:i]),
            self.buildTree(inorder[i + 1:], postorder[i:-1])
        )
```
