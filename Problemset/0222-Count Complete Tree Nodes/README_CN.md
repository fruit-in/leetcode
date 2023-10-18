# 222. 完全二叉树的节点个数
给你一棵 **完全二叉树** 的根节点 `root` ，求出该树的节点个数。

[完全二叉树](https://baike.baidu.com/item/%E5%AE%8C%E5%85%A8%E4%BA%8C%E5%8F%89%E6%A0%91/7773232?fr=aladdin) 的定义如下：在完全二叉树中，除了最底层节点可能没填满外，其余每层节点数都达到最大值，并且最下面一层的节点都集中在该层最左边的若干位置。若最底层为第 `h` 层，则该层包含 <code>1~ 2<sup>h</sup></code> 个节点。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/01/14/complete.jpg)
<pre>
<strong>输入:</strong> root = [1,2,3,4,5,6]
<strong>输出:</strong> 6
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> root = []
<strong>输出:</strong> 0
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> root = [1]
<strong>输出:</strong> 1
</pre>

#### 提示:
* 树中节点的数目范围是<code>[0, 5 * 10<sup>4</sup>]</code>
* <code>0 <= Node.val <= 5 * 10<sup>4</sup></code>
* 题目数据保证输入的树是 **完全二叉树**

**进阶：**遍历树来统计节点是一种时间复杂度为 `O(n)` 的简单解决方案。你可以设计一个更快的算法吗？

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
    def countNodes(self, root: Optional[TreeNode]) -> int:
        low = 0
        high = 1
        curr = root
        root = TreeNode(left=TreeNode(), right=root)

        while curr is not None:
            low = (low << 1) + 1
            high = (high << 1) + 1
            curr = curr.right

        while low < high:
            mid = (low + high) // 2
            curr = root
            flag = True

            for bit in bin(mid)[2:]:
                if bit == '0':
                    curr = curr.left
                else:
                    curr = curr.right

                if curr is None:
                    flag = False
                    break

            if flag:
                low = mid + 1
            else:
                high = mid

        return low - 1
```
