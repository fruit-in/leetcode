# 1302. 层数最深叶子节点的和
给你一棵二叉树，请你返回层数最深的叶子节点的和。

#### 示例:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/12/28/1483_ex1.png)
<pre>
<strong>输入:</strong> root = [1,2,3,4,5,null,6,7,null,null,null,null,8]
<strong>输出:</strong> 15
</pre>

#### 提示:
* 树中节点数目在 ```1``` 到 ```10^4``` 之间。
* 每个节点的值在 ```1``` 到 ```100``` 之间。

## 题解 (Python)

### 1. 层序遍历
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def deepestLeavesSum(self, root: TreeNode) -> int:
        curr = [root]

        while True:
            next = [n.left for n in curr if n.left]
            next.extend(n.right for n in curr if n.right)

            if not next:
                return sum(n.val for n in curr)

            curr = next
```
