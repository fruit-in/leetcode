# 437. Path Sum III
You are given a binary tree in which each node contains an integer value.

Find the number of paths that sum to a given value.

The path does not need to start or end at the root or a leaf, but it must go downwards (traveling only from parent nodes to child nodes).

The tree has no more than 1,000 nodes and the values are in the range -1,000,000 to 1,000,000.

#### Example:
<pre>
root = [10,5,-3,3,2,null,11,3,-2,null,1], sum = 8

      10
     /  \
    <strong>5   -3</strong>
   <strong>/ \    \</strong>
  <strong>3   2   11</strong>
 / \   <strong>\</strong>
3  -2   <strong>1</strong>

Return 3. The paths that sum to 8 are:

1.  5 -> 3
2.  5 -> 2 -> 1
3. -3 -> 11
</pre>

## Solutions (Python)

### 1. Recursion(Multi-Pass)
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

### 2. Iteration(Multi-Pass)
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

### 3. Recursion(One Pass)
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

### 4. Iteration(One Pass)
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
