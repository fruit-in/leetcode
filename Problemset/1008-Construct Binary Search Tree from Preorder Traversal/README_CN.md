# 1008. 前序遍历构造二叉搜索树
返回与给定前序遍历 `preorder` 相匹配的二叉搜索树（binary **search** tree）的根结点。

*(回想一下，二叉搜索树是二叉树的一种，其每个节点都满足以下规则，对于 `node.left` 的任何后代，值总 `< node.val`，而 `node.right` 的任何后代，值总 `> node.val`。此外，前序遍历首先显示节点 `node` 的值，然后遍历 `node.left`，接着遍历 `node.right`。）*

题目保证，对于给定的测试用例，总能找到满足要求的二叉搜索树。

#### 示例:
<pre>
<strong>输入:</strong> [8,5,1,7,10,12]
<strong>输出:</strong> [8,5,10,1,7,null,12]
<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/03/08/1266.png">
</pre>

#### 提示:
* `1 <= preorder.length <= 100`
* `1 <= preorder[i] <= 10^8`
* `preorder` 中的值互不相同

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
    def bstFromPreorder(self, preorder: List[int]) -> TreeNode:
        if not preorder:
            return None

        return TreeNode(
            preorder[0],
            self.bstFromPreorder(
                [val for val in preorder if val < preorder[0]]),
            self.bstFromPreorder(
                [val for val in preorder if val > preorder[0]])
        )
```
