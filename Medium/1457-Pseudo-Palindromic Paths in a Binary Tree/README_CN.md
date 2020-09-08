# 1457. 二叉树中的伪回文路径
给你一棵二叉树，每个节点的值为 1 到 9 。我们称二叉树中的一条路径是 「**伪回文**」的，当它满足：路径经过的所有节点值的排列中，存在一个回文序列。

请你返回从根到叶子节点的所有路径中 **伪回文** 路径的数目。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/05/23/palindromic_paths_1.png)
<pre>
<strong>输入:</strong> root = [2,3,1,3,1,null,1]
<strong>输出:</strong> 2
<strong>解释:</strong> 上图为给定的二叉树。总共有 3 条从根到叶子的路径：红色路径 [2,3,3] ，绿色路径 [2,1,1] 和路径 [2,3,1] 。
     在这些路径中，只有红色和绿色的路径是伪回文路径，因为红色路径 [2,3,3] 存在回文排列 [3,2,3] ，绿色路径 [2,1,1] 存在回文排列 [1,2,1] 。
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/05/23/palindromic_paths_2.png)
<pre>
<strong>输入:</strong> root = [2,1,1,1,3,null,null,null,null,null,1]
<strong>输出:</strong> 1
<strong>解释:</strong> 上图为给定二叉树。总共有 3 条从根到叶子的路径：绿色路径 [2,1,1] ，路径 [2,1,3,1] 和路径 [2,1] 。
     这些路径中只有绿色路径是伪回文路径，因为 [2,1,1] 存在回文排列 [1,2,1] 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> root = [9]
<strong>输出:</strong> 1
</pre>

#### 提示:
* 给定二叉树的节点数目在 `1` 到 `10^5` 之间。
* 节点值在 `1` 到 `9` 之间。

## 题解 (Python)

### 1. 广度优先搜索
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pseudoPalindromicPaths(self, root: TreeNode) -> int:
        def foo(root: TreeNode, x: int) -> int:
            x ^= 1 << root.val

            if not root.left and not root.right:
                return 1 if bin(x).count('1') < 2 else 0
            elif not root.left:
                return foo(root.right, x)
            elif not root.right:
                return foo(root.left, x)
            else:
                return foo(root.left, x) + foo(root.right, x)

        return foo(root, 0)
```
