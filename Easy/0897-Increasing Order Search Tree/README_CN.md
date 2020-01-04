# 897. 递增顺序查找树
给定一个树，**按中序遍历**重新排列树，使树中最左边的结点现在是树的根，并且每个结点没有左子结点，只有一个右子结点。

**示例:**
<pre>
<strong>输入:</strong> [5,3,6,2,4,null,8,1,null,null,null,7,9]

       5
      / \
    3    6
   / \    \
  2   4    8
 /        / \
1        7   9

<strong>输出:</strong> [1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]

 1
  \
   2
    \
     3
      \
       4
        \
         5
          \
           6
            \
             7
              \
               8
                \
                 9
</pre>

#### 提示:
1. 给定树中的结点数介于 1 和 100 之间。
2. 每个结点都有一个从 0 到 1000 范围内的唯一整数值。

## 题解 (Python)

### 1. 中序遍历（构造新树）
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def increasingBST(self, root: TreeNode) -> TreeNode:
        root_parent = TreeNode(0)
        prev = root_parent
        nodes = []
        curr = root

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()

            prev.right = curr
            curr.left = None
            prev = curr

            curr = curr.right

        return root_parent.right
```

### 2. 中序遍历（更改连接）
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def increasingBST(self, root: TreeNode) -> TreeNode:
        prev = None
        nodes = []
        curr = root

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.right

            curr = nodes.pop()

            new_node = TreeNode(curr.val)
            new_node.right = prev
            prev = new_node

            curr = curr.left

        return prev
```
