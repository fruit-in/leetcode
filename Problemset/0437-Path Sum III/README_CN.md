# 437. 路径总和 III
给定一个二叉树，它的每个结点都存放着一个整数值。

找出路径和等于给定数值的路径总数。

路径不需要从根节点开始，也不需要在叶子节点结束，但是路径方向必须是向下的（只能从父节点到子节点）。

二叉树不超过1000个节点，且节点数值范围是 [-1000000,1000000] 的整数。

#### 示例:
<pre>
root = [10,5,-3,3,2,null,11,3,-2,null,1], sum = 8

      10
     /  \
    <strong>5   -3</strong>
   <strong>/ \    \</strong>
  <strong>3   2   11</strong>
 / \   <strong>\</strong>
3  -2   <strong>1</strong>

返回 3。和等于 8 的路径有:

1.  5 -> 3
2.  5 -> 2 -> 1
3. -3 -> 11
</pre>

## 题解 (Python)

### 1. 递归（多次遍历）
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> int:
        def rootPathSum(root: TreeNode, sum: int) -> int:
            if not root:
                return 0

            ret = 1 if root.val == sum else 0

            ret += rootPathSum(root.left, sum - root.val)
            ret += rootPathSum(root.right, sum - root.val)

            return ret


        if not root:
            return 0

        ret = rootPathSum(root, sum)

        ret += self.pathSum(root.left, sum)
        ret += self.pathSum(root.right, sum)

        return ret
```

### 2. 迭代（多次遍历）
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> int:
        nodes = [root]
        nodes_sum = []
        ret = 0

        while nodes:
            cur = nodes.pop()
            if cur:
                nodes.append(cur.left)
                nodes.append(cur.right)
                nodes_sum.append((cur, sum))

        while nodes_sum:
            cur, sum = nodes_sum.pop()
            if cur.val == sum:
                ret += 1
            if cur.left:
                nodes_sum.append((cur.left, sum - cur.val))
            if cur.right:
                nodes_sum.append((cur.right, sum - cur.val))

        return ret
```

### 3. 递归（单次遍历）
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> int:
        def helper(root: TreeNode, path_sum: dict, prev_sum: int, sum: int) -> int:
            if not root:
                return 0

            curr_sum = prev_sum + root.val
            ret = 0

            if path_sum.get(curr_sum - sum):
                ret += path_sum.get(curr_sum - sum)

            if not path_sum.get(curr_sum):
                path_sum[curr_sum] = 0
            path_sum[curr_sum] += 1

            ret += helper(root.left, dict(path_sum), curr_sum, sum)
            ret += helper(root.right, dict(path_sum), curr_sum, sum)

            return ret


        return helper(root, {0: 1}, 0, sum)
```

### 4. 迭代（单次遍历）
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> int:
        if not root:
            return 0

        nodes_sum = [(root, [sum])]
        ret = 0

        while nodes_sum:
            cur, s = nodes_sum.pop()

            ret += s.count(cur.val)

            s = [x - cur.val for x in s]
            s.append(sum)

            if cur.left:
                nodes_sum.append((cur.left, s))
            if cur.right:
                nodes_sum.append((cur.right, s))

        return ret
```
