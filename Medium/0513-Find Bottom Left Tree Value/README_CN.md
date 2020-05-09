# 513. 找树左下角的值
给定一个二叉树，在树的最后一行找到最左边的值。

#### 示例 1:
<pre>
<strong>输入:</strong>
    2
   / \
  1   3
<strong>输出:</strong>
1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
        1
       / \
      2   3
     /   / \
    4   5   6
       /
      7
<strong>输出:</strong>
7
</pre>

#### 注意:
您可以假设树（即给定的根节点）不为 **NULL**。

## 题解 (Python)

### 1. 广度优先搜索
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findBottomLeftValue(self, root: TreeNode) -> int:
        curr_level = [root]

        while curr_level:
            ret = curr_level[0].val
            next_level = [node.left for node in curr_level if node.left]
            next_level.extend(node.right for node in curr_level if node.right)
            curr_level = next_level

        return ret
```

### 2. 深度优先搜索
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findBottomLeftValue(self, root: TreeNode) -> int:
        def dfs(root: TreeNode, depth: int) -> (int, int):
            ori_depth = depth
            val, depth = root.val, depth

            if root.left:
                val, depth = dfs(root.left, ori_depth + 1)
            if root.right:
                right_val, right_depth = dfs(root.right, ori_depth + 1)
                if right_depth > depth:
                    val, depth = right_val, right_depth

            return val, depth

        return dfs(root, 1)[0]
```
