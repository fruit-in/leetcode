# 590. N叉树的后序遍历
给定一个 N 叉树，返回其节点值的*后序遍历*。

例如，给定一个 ```3叉树``` :
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/narytreeexample.png)

返回其后序遍历: ```[5,6,3,2,4,1]```.

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
    def postorder(self, root: 'Node') -> List[int]:
        if not root:
            return []
        vals = []
        for n in root.children:
            vals.extend(self.postorder(n))
        vals.append(root.val)
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
    def postorder(self, root: 'Node') -> List[int]:
        if not root:
            return []
        nset = set()
        vals = []
        nodes = [root]
        while nodes:
            cur = nodes.pop()
            if not cur.children or cur.children[0] in nset:
                vals.append(cur.val)
                nset.add(cur)
            else:
                nodes.append(cur)
                for child in cur.children[::-1]:
                    nodes.append(child)
        return vals
```
