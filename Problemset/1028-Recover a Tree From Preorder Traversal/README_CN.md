# 1028. 从先序遍历还原二叉树
我们从二叉树的根节点 ```root``` 开始进行深度优先搜索。

在遍历中的每个节点处，我们输出 ```D``` 条短划线（其中 ```D``` 是该节点的深度），然后输出该节点的值。*（如果节点的深度为 ```D```，则其直接子节点的深度为 ```D + 1```。根节点的深度为 ```0```）*。

如果节点只有一个子节点，那么保证该子节点为左子节点。

给出遍历输出 ```S```，还原树并返回其根节点 ```root```。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/04/12/recover-a-tree-from-preorder-traversal.png)
<pre>
<strong>输入:</strong> "1-2--3--4-5--6--7"
<strong>输出:</strong> [1,2,5,3,4,6,7]
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/04/12/screen-shot-2019-04-10-at-114101-pm.png)
<pre>
<strong>输入:</strong> "1-2--3---4-5--6---7"
<strong>输出:</strong> [1,2,5,3,null,6,null,4,null,7]
</pre>

#### 示例 3:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/04/12/screen-shot-2019-04-10-at-114955-pm.png)
<pre>
<strong>输入:</strong> "1-401--349---90--88"
<strong>输出:</strong> [1,401,null,349,88,90]
</pre>

#### 提示:
* 原始树中的节点数介于 ```1``` 和 ```1000``` 之间。
* 每个节点的值介于 ```1``` 和 ```10 ^ 9``` 之间。

## 题解 (Python)

### 1. 栈
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def recoverFromPreorder(self, S: str) -> TreeNode:
        vals = [int(n) for n in S.split('-') if n != '']
        depths = [0]
        depth = 0

        for ch in S:
            if ch == '-':
                depth += 1
            elif depth != 0:
                depths.append(depth)
                depth = 0

        stack = []

        while vals:
            node = TreeNode(vals.pop(0))
            depth = depths.pop(0)

            while stack and stack[-1][1] >= depth:
                stack.pop()

            if stack and not stack[-1][0].left:
                stack[-1][0].left = node
            elif stack and not stack[-1][0].right:
                stack[-1][0].right = node

            stack.append((node, depth))

        return stack[0][0]
```
