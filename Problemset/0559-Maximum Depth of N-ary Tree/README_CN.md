# 559. N叉树的最大深度
给定一个 N 叉树，找到其最大深度。

最大深度是指从根节点到最远叶子节点的最长路径上的节点总数。

例如，给定一个 ```3叉树``` :<br>
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/narytreeexample.png)<br>
我们应返回其最大深度，3。

#### 说明:
1. 树的深度不会超过 ```1000```。
2. 树的节点总不会超过 ```5000```。

## 题解 (Python)

### 1. 深度优先搜索
```Python3
"""
# Definition for a Node.
class Node:
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""

class Solution:
    def maxDepth(self, root: 'Node') -> int:
        if not root:
            return 0
        if not root.children:
            return 1
        return max(map(self.maxDepth, root.children)) + 1
```

### 2. 广度优先搜索
```Python3
"""
# Definition for a Node.
class Node:
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""

class Solution:
    def maxDepth(self, root: 'Node') -> int:
        if not root:
            return 0
        depth = 1
        nodes = [n for n in root.children]
        while nodes:
            depth += 1
            nodes = [n for node in nodes for n in node.children]
        return depth
```
