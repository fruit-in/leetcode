# 429. N叉树的层序遍历
给定一个 N 叉树，返回其节点值的*层序遍历*。 (即从左到右，逐层遍历)。

例如，给定一个 ```3叉树``` :

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/narytreeexample.png)

返回其层序遍历:
```
[
     [1],
     [3,2,4],
     [5,6]
]
```

#### 说明:
1. 树的深度不会超过 ```1000```。
2. 树的节点总数不会超过 ```5000```。

## 题解 (Python)

### 1. 广度优先搜索
```Python3
"""
# Definition for a Node.
class Node:
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""
class Solution:
    def levelOrder(self, root: 'Node') -> List[List[int]]:
        if not root:
            return []
        ret = []
        nodes = [root]
        while nodes:
            level = []
            temp = []
            for cur in nodes:
                level.append(cur.val)
                temp.extend(cur.children)
            nodes = temp
            ret.append(level)
        return ret
```
