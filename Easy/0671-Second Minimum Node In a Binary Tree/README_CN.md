# 671. 二叉树中第二小的节点
给定一个非空特殊的二叉树，每个节点都是正数，并且每个节点的子节点数量只能为 ```2``` 或 ```0```。如果一个节点有两个子节点的话，那么这个节点的值不大于它的子节点的值。

给出这样的一个二叉树，你需要输出所有节点中的**第二小的值**。如果第二小的值不存在的话，输出 -1 。

#### 示例 1:
<pre>
<strong>输入:</strong>
    2
   / \
  2   5
     / \
    5   7
<strong>输出:</strong> 5
<strong>说明:</strong> 最小的值是 2 ，第二小的值是 5 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
    2
   / \
  2   2
<strong>输出:</strong> -1
<strong>说明:</strong> 最小的值是 2, 但是不存在第二小的值。
</pre>

## 题解 (Python)

### 1. 暴力法
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findSecondMinimumValue(self, root: TreeNode) -> int:
        vals = set()
        nodes = [root]

        while nodes:
            curr = nodes.pop()
            vals.add(curr.val)
            if curr.left:
                nodes.append(curr.left)
                nodes.append(curr.right)

        vals.remove(min(vals))

        return min(vals) if vals else -1
```

### 2. 存储最小和第二小的值
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findSecondMinimumValue(self, root: TreeNode) -> int:
        ret = float('+inf')
        nodes = [root]

        while nodes:
            curr = nodes.pop()
            if root.val < curr.val < ret:
                ret = curr.val
            elif curr.val == root.val and curr.left:
                nodes.append(curr.left)
                nodes.append(curr.right)

        return ret if ret < float('+inf') else -1
```
