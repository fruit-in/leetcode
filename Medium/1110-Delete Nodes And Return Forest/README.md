# 1110. Delete Nodes And Return Forest
Given the `root` of a binary tree, each node in the tree has a distinct value.

After deleting all nodes with a value in `to_delete`, we are left with a forest (a disjoint union of trees).

Return the roots of the trees in the remaining forest. You may return the result in any order.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/07/01/screen-shot-2019-07-01-at-53836-pm.png)
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,6,7], to_delete = [3,5]
<strong>Output:</strong> [[1,2,null,4],[6],[7]]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> root = [1,2,4,null,3], to_delete = [3]
<strong>Output:</strong> [[1,2,4]]
</pre>

#### Constraints:
* The number of nodes in the given tree is at most `1000`.
* Each node has a distinct value between `1` and `1000`.
* `to_delete.length <= 1000`
* `to_delete` contains distinct values between `1` and `1000`.

## Solutions (Python)

### 1. Solution
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def delNodes(self, root: TreeNode, to_delete: List[int]) -> List[TreeNode]:
        if root is None:
            return []

        ret = [root]
        left_ret = self.delNodes(root.left, to_delete)
        right_ret = self.delNodes(root.right, to_delete)

        if root.val in to_delete:
            return left_ret + right_ret

        if root.left is not None:
            if root.left.val in to_delete:
                root.left = None
                ret += left_ret
            else:
                ret += left_ret[1:]
        if root.right is not None:
            if root.right.val in to_delete:
                root.right = None
                ret += right_ret
            else:
                ret += right_ret[1:]

        return ret
```

## Solutions (Ruby)

### 1. Solution
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
# @param {Integer[]} to_delete
# @return {TreeNode[]}
def del_nodes(root, to_delete)
  return [] if root.nil?

  ret = [root]
  left_ret = del_nodes(root.left, to_delete)
  right_ret = del_nodes(root.right, to_delete)

  return left_ret + right_ret if to_delete.include?(root.val)

  unless root.left.nil?
    if to_delete.include?(root.left.val)
      root.left = nil
      ret += left_ret
    else
      ret += left_ret[1..]
    end
  end
  unless root.right.nil?
    if to_delete.include?(root.right.val)
      root.right = nil
      ret += right_ret
    else
      ret += right_ret[1..]
    end
  end

  ret
end
```
