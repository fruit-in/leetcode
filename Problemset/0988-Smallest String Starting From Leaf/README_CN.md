# 988. 从叶结点开始的最小字符串
给定一颗根结点为 `root` 的二叉树，树中的每一个结点都有一个 `[0, 25]` 范围内的值，分别代表字母 `'a'` 到 `'z'`。

返回 ***按字典序最小** 的字符串，该字符串从这棵树的一个叶结点开始，到根结点结束*。

注：字符串中任何较短的前缀在 **字典序上** 都是 **较小** 的：

* 例如，在字典序上 `"ab"` 比 `"aba"` 要小。叶结点是指没有子结点的结点。

节点的叶节点是没有子节点的节点。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/01/30/tree1.png)
<pre>
<strong>输入:</strong> root = [0,1,2,3,4,3,4]
<strong>输出:</strong> "dba"
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2019/01/30/tree2.png)
<pre>
<strong>输入:</strong> root = [25,1,3,1,3,0,2]
<strong>输出:</strong> "adz"
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2019/02/01/tree3.png)
<pre>
<strong>输入:</strong> root = [2,2,1,null,1,0,null,0]
<strong>输出:</strong> "abc"
</pre>

#### 提示:
* 给定树的结点数在 `[1, 8500]` 范围内
* `0 <= Node.val <= 25`

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
    def smallestFromLeaf(self, root: Optional[TreeNode]) -> str:
        return self.dfs("", root)

    def dfs(self, s: str, root: Optional[TreeNode]) -> str:
        if root is None:
            return s

        s = chr(root.val + 97) + s

        if root.left is None:
            return self.dfs(s, root.right)
        elif root.right is None:
            return self.dfs(s, root.left)
        else:
            return min(self.dfs(s, root.left), self.dfs(s, root.right))
```
