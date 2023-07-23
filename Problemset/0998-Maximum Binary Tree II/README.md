# 998. Maximum Binary Tree II
We are given the `root` node of a *maximum tree*: a tree where every node has a value greater than any other value in its subtree.

Just as in the [previous problem](https://leetcode.com/problems/maximum-binary-tree/), the given tree was constructed from an list `A` (`root = Construct(A)`) recursively with the following `Construct(A)` routine:
* If `A` is empty, return `null`.
* Otherwise, let `A[i]` be the largest element of `A`.  Create a `root` node with value `A[i]`.
* The left child of `root` will be `Construct([A[0], A[1], ..., A[i-1]])`
* The right child of `root` will be `Construct([A[i+1], A[i+2], ..., A[A.length - 1]])`
* Return `root`.

Note that we were not given A directly, only a root node `root = Construct(A)`.

Suppose `B` is a copy of `A` with the value `val` appended to it.  It is guaranteed that `B` has unique values.

Return `Construct(B)`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/02/21/maximum-binary-tree-1-1.png) ![](https://assets.leetcode.com/uploads/2019/02/21/maximum-binary-tree-1-2.png)
<pre>
<strong>Input:</strong> root = [4,1,3,null,null,2], val = 5
<strong>Output:</strong> [5,4,null,1,3,null,null,2]
<strong>Explanation:</strong> A = [1,4,2,3], B = [1,4,2,3,5]
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/02/21/maximum-binary-tree-2-1.png) ![](https://assets.leetcode.com/uploads/2019/02/21/maximum-binary-tree-2-2.png)
<pre>
<strong>Input:</strong> root = [5,2,4,null,1], val = 3
<strong>Output:</strong> [5,2,4,null,1,null,3]
<strong>Explanation:</strong> A = [2,1,5,4], B = [2,1,5,4,3]
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2019/02/21/maximum-binary-tree-3-1.png) ![](https://assets.leetcode.com/uploads/2019/02/21/maximum-binary-tree-3-2.png)
<pre>
<strong>Input:</strong> root = [5,2,3,null,1], val = 4
<strong>Output:</strong> [5,2,4,null,1,3]
<strong>Explanation:</strong> A = [2,1,5,3], B = [2,1,5,3,4]
</pre>

#### Constraints:
* `1 <= B.length <= 100`

## Solutions (Python)

### 1. Recursion
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def insertIntoMaxTree(self, root: TreeNode, val: int) -> TreeNode:
        if root is None:
            return TreeNode(val)
        elif root.val < val:
            return TreeNode(val, root)
        else:
            root.right = self.insertIntoMaxTree(root.right, val)
            return root
```

## Solutions (Ruby)

### 1. Recursion
```Ruby
# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
# @param {TreeNode} root
# @param {Integer} val
# @return {TreeNode}
def insert_into_max_tree(root, val)
  if root.nil?
    TreeNode.new(val)
  elsif root.val < val
    TreeNode.new(val, root)
  else
    root.right = insert_into_max_tree(root.right, val)
    root
  end
end
```
