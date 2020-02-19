# 1315. 祖父节点值为偶数的节点和
给你一棵二叉树，请你返回满足以下条件的所有节点的值之和：
* 该节点的祖父节点的值为偶数。（一个节点的祖父节点是指该节点的父节点的父节点。）

如果不存在祖父节点值为偶数的节点，那么返回 ```0``` 。

#### 示例:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/01/10/1473_ex1.png)
<pre>
<strong>输入:</strong> root = [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
<strong>输出:</strong> 18
<strong>解释:</strong> 图中红色节点的祖父节点的值为偶数，蓝色节点为这些红色节点的祖父节点。
</pre>

#### 提示:
* 树中节点的数目在 ```1``` 到 ```10^4``` 之间。
* 每个节点的值在 ```1``` 到 ```100``` 之间。

## 题解 (Python)

### 1. 深度优先搜索
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def sumEvenGrandparent(self, root: TreeNode) -> int:
        stack = [root]
        ret = 0

        while stack:
            curr = stack.pop()

            if curr.left:
                stack.append(curr.left)
                if curr.val % 2 == 0:
                    ret += curr.left.left.val if curr.left.left else 0
                    ret += curr.left.right.val if curr.left.right else 0
            if curr.right:
                stack.append(curr.right)
                if curr.val % 2 == 0:
                    ret += curr.right.left.val if curr.right.left else 0
                    ret += curr.right.right.val if curr.right.right else 0

        return ret
```
