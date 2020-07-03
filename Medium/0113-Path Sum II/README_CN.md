# 113. 路径总和 II
给定一个二叉树和一个目标和，找到所有从根节点到叶子节点路径总和等于给定目标和的路径。

**说明:** 叶子节点是指没有子节点的节点。

#### 示例:
给定如下二叉树，以及目标和 `sum = 22`，
<pre>
              <b>5</b>
             <b>/ \</b>
            <b>4   8</b>
           <b>/</b>   / <b>\</b>
          <b>11</b>  13  <b>4</b>
         /  <b>\    /</b> \
        7    <b>2  5</b>   1
</pre>

返回:
```
[
   [5,4,11,2],
   [5,8,4,5]
]
```

## 题解 (Python)

### 1. 题解
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> List[List[int]]:
        if not root:
            return []

        if not (root.left or root.right) and root.val == sum:
            return [[root.val]]

        paths = self.pathSum(root.left, sum - root.val)
        paths.extend(self.pathSum(root.right, sum - root.val))

        for path in paths:
            path.insert(0, root.val)

        return paths
```
