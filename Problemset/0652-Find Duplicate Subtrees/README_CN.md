# 652. 寻找重复的子树
给你一棵二叉树的根节点 `root` ，返回所有 **重复的子树** 。

对于同一类的重复子树，你只需要返回其中任意 **一棵** 的根结点即可。

如果两棵树具有 **相同的结构** 和 **相同的结点值** ，则认为二者是 **重复** 的。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/08/16/e1.jpg)
<pre>
<strong>输入:</strong> root = [1,2,3,4,null,2,4,null,null,4]
<strong>输出:</strong> [[2,4],[4]]
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/08/16/e2.jpg)
<pre>
<strong>输入:</strong> root = [2,1,1]
<strong>输出:</strong> [[1]]
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2020/08/16/e33.jpg)
<pre>
<strong>输入:</strong> root = [2,2,2,3,null,3,null]
<strong>输出:</strong> [[2,3],[3]]
</pre>

#### 提示:
* 树中的结点数在 `[1, 5000]` 范围内。
* `-200 <= Node.val <= 200`

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
    def findDuplicateSubtrees(self, root: Optional[TreeNode]) -> List[Optional[TreeNode]]:
        count = {}
        ret = []

        def dfs(root: Optional[TreeNode]) -> str:
            if root is None:
                return "()"

            s = "{}({})({})".format(root.val, dfs(root.left), dfs(root.right))
            count[s] = count.get(s, 0) + 1

            if count[s] == 2:
                ret.append(root)

            return s

        dfs(root)

        return ret
```
