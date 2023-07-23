# 450. 删除二叉搜索树中的节点
给定一个二叉搜索树的根节点 **root** 和一个值 **key**，删除二叉搜索树中的 **key** 对应的节点，并保证二叉搜索树的性质不变。返回二叉搜索树（有可能被更新）的根节点的引用。

一般来说，删除节点可分为两个步骤：
1. 首先找到需要删除的节点；
2. 如果找到了，删除它。

**说明:** 要求算法时间复杂度为 O(h)，h 为树的高度。

#### 示例:
```
root = [5,3,6,2,4,null,7]
key = 3

    5
   / \
  3   6
 / \   \
2   4   7

给定需要删除的节点值是 3，所以我们首先找到 3 这个节点，然后删除它。

一个正确的答案是 [5,4,6,2,null,null,7], 如下图所示。

    5
   / \
  4   6
 /     \
2       7

另一个正确答案是 [5,2,6,null,4,null,7]。

    5
   / \
  2   6
   \   \
    4   7
```

## 题解 (Python)

### 1. 深度优先搜索
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def deleteNode(self, root: TreeNode, key: int) -> TreeNode:
        if root is None:
            return None

        if root.val == key:
            root = self.merge(root.left, root.right)
        elif root.val > key:
            root.left = self.deleteNode(root.left, key)
        else:
            root.right = self.deleteNode(root.right, key)

        return root

    def merge(self, left: TreeNode, right: TreeNode) -> TreeNode:
        if left is None:
            return right

        curr = left
        while curr.right is not None:
            curr = curr.right
        curr.right = right

        return left
```

## 题解 (Ruby)

### 1. 深度优先搜索
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
# @param {Integer} key
# @return {TreeNode}
def delete_node(root, key)
  return nil if root.nil?

  if root.val == key
    root = merge(root.left, root.right)
  elsif root.val > key
    root.left = delete_node(root.left, key)
  else
    root.right = delete_node(root.right, key)
  end

  root
end

# @param {TreeNode} left
# @param {TreeNode} right
# @return {TreeNode}
def merge(left, right)
  return right if left.nil?

  curr = left
  curr = curr.right until curr.right.nil?
  curr.right = right

  left
end
```
