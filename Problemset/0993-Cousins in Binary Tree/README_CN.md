# 993. 二叉树的堂兄弟节点
在二叉树中，根节点位于深度 ```0``` 处，每个深度为 ```k``` 的节点的子节点位于深度 ```k+1``` 处。

如果二叉树的两个节点深度相同，但**父节点不同**，则它们是一对*堂兄弟节点*。

我们给出了具有唯一值的二叉树的根节点 ```root```，以及树中两个不同节点的值 ```x``` 和 ```y```。

只有与值 ```x``` 和 ```y``` 对应的节点是堂兄弟节点时，才返回 ```true```。否则，返回 ```false```。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/02/16/q1248-01.png)
<pre>
<strong>输入:</strong> root = [1,2,3,4], x = 4, y = 3
<strong>输出:</strong> false
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/02/16/q1248-02.png)
<pre>
<strong>输入:</strong> root = [1,2,3,null,4,null,5], x = 5, y = 4
<strong>输出:</strong> true
</pre>

#### 示例 3:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/02/16/q1248-03.png)
<pre>
<strong>输入:</strong> root = [1,2,3,null,4], x = 2, y = 3
<strong>输出:</strong> false
</pre>

#### 提示:
1. 二叉树的节点数介于 ```2``` 到 ```100``` 之间。
2. 每个节点的值都是唯一的、范围为 ```1``` 到 ```100``` 的整数。

## 题解 (Python)

### 1. 深度优先搜索
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isCousins(self, root: TreeNode, x: int, y: int) -> bool:
        parent, depth = {root.val: None}, {}
        def dfs(root: TreeNode, dep: int):
            depth[root.val] = dep
            if root.left:
                parent[root.left.val] = root
                dfs(root.left, dep + 1)
            if root.right:
                parent[root.right.val] = root
                dfs(root.right, dep + 1)

        dfs(root, 0)
        return depth[x] == depth[y] and parent[x] != parent[y]
```

### 2. 广度优先搜索
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isCousins(self, root: TreeNode, x: int, y: int) -> bool:
        nodes = [root]
        while nodes:
            nodes = [node.left for node in nodes if node] \
                + [node.right for node in nodes if node]
            vals = [node.val if node else 0 for node in nodes]
            if x in vals and y in vals:
                return abs(vals.index(x) - vals.index(y)) != len(vals) / 2
            elif x in vals or y in vals:
                return False
```
