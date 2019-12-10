# 589. N叉树的前序遍历
给定一个 N 叉树，返回其节点值的*前序遍历*。

例如，给定一个 ```3叉树```:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/narytreeexample.png)

返回其前序遍历: ```[1,3,5,6,2,4]```。

**说明:** 递归法很简单，你可以使用迭代法完成此题吗?

## 题解 (Python)

### 1. 递归
```Python3
"""
# Definition for a Node.
class Node:
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""
class Solution:
    def preorder(self, root: 'Node') -> List[int]:
        if not root:
            return []
        vals = [root.val]
        for child in root.children:
            vals.extend(self.preorder(child))
        return vals
```

### 2. 迭代
```Python3
"""
# Definition for a Node.
class Node:
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""
class Solution:
    def preorder(self, root: 'Node') -> List[int]:
        vals = []
        nodes = [root]
        while nodes:
            cur = nodes.pop()
            if cur:
                vals.append(cur.val)
                for n in cur.children[::-1]:
                    nodes.append(n)
        return vals
```
