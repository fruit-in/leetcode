# 450. Delete Node in a BST
Given a root node reference of a BST and a key, delete the node with the given key in the BST. Return the root node reference (possibly updated) of the BST.

Basically, the deletion can be divided into two stages:
1. Search for a node to remove.
2. If the node is found, delete the node.

**Follow up:** Can you solve it with time complexity `O(height of tree)`?

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/04/del_node_1.jpg)
<pre>
<strong>Input:</strong> root = [5,3,6,2,4,null,7], key = 3
<strong>Output:</strong> [5,4,6,2,null,null,7]
<strong>Explanation:</strong> Given key to delete is 3. So we find the node with value 3 and delete it.
One valid answer is [5,4,6,2,null,null,7], shown in the above BST.
Please notice that another valid answer is [5,2,6,null,4,null,7] and it's also accepted.
<img src="https://assets.leetcode.com/uploads/2020/09/04/del_node_supp.jpg">
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> root = [5,3,6,2,4,null,7], key = 0
<strong>Output:</strong> [5,3,6,2,4,null,7]
<strong>Explanation:</strong> The tree does not contain a node with value = 0.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> root = [], key = 0
<strong>Output:</strong> []
</pre>

#### Constraints:
* The number of nodes in the tree is in the range <code>[0, 10<sup>4</sup>]</code>.
* <code>-10<sup>5</sup> <= Node.val <= 10<sup>5</sup></code>
* Each node has a **unique** value.
* `root` is a valid binary search tree.
* <code>-10<sup>5</sup> <= key <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. DFS
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

## Solutions (Ruby)

### 1. DFS
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
