# 662. 二叉树最大宽度
给你一棵二叉树的根节点 `root` ，返回树的 **最大宽度** 。

树的 **最大宽度** 是所有层中最大的 **宽度** 。

每一层的 **宽度** 被定义为该层最左和最右的非空节点（即，两个端点）之间的长度。将这个二叉树视作与满二叉树结构相同，两端点间会出现一些延伸到这一层的 `null` 节点，这些 `null` 节点也计入长度。

题目数据保证答案将会在  **32 位** 带符号整数范围内。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/05/03/width1-tree.jpg)
<pre>
<strong>输入:</strong> root = [1,3,2,5,3,null,9]
<strong>输出:</strong> 4
<strong>解释:</strong> 最大宽度出现在树的第 3 层，宽度为 4 (5,3,null,9) 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/03/14/maximum-width-of-binary-tree-v3.jpg)
<pre>
<strong>输入:</strong> root = [1,3,2,5,null,null,9,6,null,7]
<strong>输出:</strong> 7
<strong>解释:</strong> 最大宽度出现在树的第 4 层，宽度为 7 (6,null,null,null,null,null,7) 。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/05/03/width3-tree.jpg)
<pre>
<strong>输入:</strong> root = [1,3,2,5]
<strong>输出:</strong> 2
<strong>解释:</strong> 最大宽度出现在树的第 2 层，宽度为 2 (3,2) 。
</pre>

#### 提示:
* 树中节点的数目范围是 `[1, 3000]`
* `-100 <= Node.val <= 100`

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
    def widthOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        currlevel = [(root, 0)]
        ret = 1

        while currlevel != []:
            nextlevel = []
            ret = max(ret, currlevel[-1][1] - currlevel[0][1] + 1)

            for node, x in currlevel:
                if node.left is not None:
                    nextlevel.append((node.left, x << 1))
                if node.right is not None:
                    nextlevel.append((node.right, (x << 1) + 1))

            currlevel = nextlevel

        return ret
```
