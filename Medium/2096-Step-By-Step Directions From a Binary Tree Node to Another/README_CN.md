# 2096. 从二叉树一个节点到另一个节点每一步的方向
给你一棵 **二叉树** 的根节点 `root` ，这棵二叉树总共有 `n` 个节点。每个节点的值为 `1` 到 `n` 中的一个整数，且互不相同。给你一个整数 `startValue` ，表示起点节点 `s` 的值，和另一个不同的整数 `destValue` ，表示终点节点 `t` 的值。

请找到从节点 `s` 到节点 `t` 的 **最短路径** ，并以字符串的形式返回每一步的方向。每一步用 **大写** 字母 `'L'` ，`'R'` 和 `'U'` 分别表示一种方向：

* `'L'` 表示从一个节点前往它的 **左孩子** 节点。
* `'R'` 表示从一个节点前往它的 **右孩子** 节点。
* `'U'` 表示从一个节点前往它的 **父** 节点。

请你返回从 `s` 到 `t` **最短路径** 每一步的方向。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/11/15/eg1.png)
<pre>
<strong>输入:</strong> root = [5,1,2,3,null,6,4], startValue = 3, destValue = 6
<strong>输出:</strong> "UURL"
<strong>解释:</strong> 最短路径为：3 → 1 → 5 → 2 → 6 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/11/15/eg2.png)
<pre>
<strong>输入:</strong> root = [2,1], startValue = 2, destValue = 1
<strong>输出:</strong> "L"
<strong>解释:</strong> 最短路径为：2 → 1 。
</pre>

#### 提示:
* 树中节点数目为 `n` 。
* <code>2 <= n <= 10<sup>5</sup></code>
* `1 <= Node.val <= n`
* 树中所有节点的值 **互不相同** 。
* `1 <= startValue, destValue <= n`
* `startValue != destValue`

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
    def getDirections(self, root: Optional[TreeNode], startValue: int, destValue: int) -> str:
        root2Start = self.root2(root, startValue)
        root2Dest = self.root2(root, destValue)
        minPathLen = min(len(root2Start), len(root2Dest))

        for i in range(minPathLen + 1):
            if i == minPathLen or root2Start[i] != root2Dest[i]:
                return 'U' * (len(root2Start) - i) + root2Dest[i:]

    def root2(self, root: Optional[TreeNode], destValue: int) -> Optional[str]:
        if root is None:
            return None

        if root.val == destValue:
            return ""

        root2Left = self.root2(root.left, destValue)
        if root2Left is not None:
            return 'L' + root2Left

        root2Right = self.root2(root.right, destValue)
        if root2Right is not None:
            return 'R' + root2Right

        return None
```
