# 958. 二叉树的完全性检验
给你一棵二叉树的根节点 `root` ，请你判断这棵树是否是一棵 **完全二叉树** 。

在一棵 [**完全二叉树**](https://baike.baidu.com/item/%E5%AE%8C%E5%85%A8%E4%BA%8C%E5%8F%89%E6%A0%91/7773232?fr=aladdin) 中，除了最后一层外，所有层都被完全填满，并且最后一层中的所有节点都尽可能靠左。最后一层（第 `h` 层）中可以包含 `1` 到 <code>2<sup>h</sup></code> 个节点。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2018/12/15/complete-binary-tree-1.png)
<pre>
<strong>输入:</strong> root = [1,2,3,4,5,6]
<strong>输出:</strong> true
<strong>解释:</strong> 最后一层前的每一层都是满的（即，节点值为 {1} 和 {2,3} 的两层），且最后一层中的所有节点（{4,5,6}）尽可能靠左。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2018/12/15/complete-binary-tree-2.png)
<pre>
<strong>输入:</strong> root = [1,2,3,4,5,null,7]
<strong>输出:</strong> false
<strong>解释:</strong> 值为 7 的节点不满足条件「节点尽可能靠左」。
</pre>

#### 提示:
* 树中节点数目在范围 `[1, 100]` 内
* `1 <= Node.val <= 1000`

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
    def isCompleteTree(self, root: Optional[TreeNode]) -> bool:
        nodes = [root]
        depth = 0

        while True:
            children = []

            if nodes[0] == None:
                return True

            if len(nodes) != 2 ** depth:
                return False

            for node in nodes:
                if node is not None:
                    children.append(node.left)
                    children.append(node.right)

            for i in range(1, len(children)):
                if children[i - 1] is None and children[i] is not None:
                    return False

            nodes = children
            depth += 1
```
