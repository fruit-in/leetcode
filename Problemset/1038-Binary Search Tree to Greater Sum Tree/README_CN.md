# 1038. 从二叉搜索树到更大和树
给出二叉**搜索**树的根节点，该二叉树的节点值各不相同，修改二叉树，使每个节点 ```node``` 的新值等于原树中大于或等于 ```node.val``` 的值之和。

提醒一下，二叉搜索树满足下列约束条件：
* 节点的左子树仅包含键**小于**节点键的节点。
* 节点的右子树仅包含键**大于**节点键的节点。
* 左右子树也必须是二叉搜索树。

#### 示例:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/05/03/tree.png)
<pre>
<strong>输入:</strong> [4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
<strong>输出:</strong> [30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]
</pre>

#### 提示:
1. 树中的节点数介于 ```1``` 和 ```100``` 之间。
2. 每个节点的值介于 ```0``` 和 ```100``` 之间。
3. 给定的树为二叉搜索树。

## 题解 (Python)

### 1. 中序遍历
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def bstToGst(self, root: TreeNode) -> TreeNode:
        stack = []
        curr = root
        sum = 0

        while stack or curr:
            while curr:
                stack.append(curr)
                curr = curr.right

            curr = stack.pop()
            sum += curr.val
            curr.val = sum

            curr = curr.left

        return root
```
