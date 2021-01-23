# 1367. Linked List in Binary Tree
Given a binary tree `root` and a linked list with `head` as the first node.

Return True if all the elements in the linked list starting from the `head` correspond to some *downward path* connected in the binary tree otherwise return False.

In this context downward path means a path that starts at some node and goes downwards.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/02/12/sample_1_1720.png)
<pre>
<strong>Input:</strong> head = [4,2,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
<strong>Output:</strong> true
<strong>Explanation:</strong> Nodes in blue form a subpath in the binary Tree.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/02/12/sample_2_1720.png)
<pre>
<strong>Input:</strong> head = [1,4,2,6], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
<strong>Output:</strong> true
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> head = [1,4,2,6,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
<strong>Output:</strong> false
<strong>Explanation:</strong> There is no path in the binary tree that contains all the elements of the linked list from head.
</pre>

#### Constraints:
* The number of nodes in the tree will be in the range `[1, 2500]`.
* The number of nodes in the list will be in the range `[1, 100]`.
* `1 <= Node.val <= 100` for each node in the linked list and binary tree.

## Solutions (Python)

### 1. Recursion
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isSubPath(self, head: ListNode, root: TreeNode) -> bool:
        if root is None:
            return False
        elif head.val == root.val and self.checkPath(head, root):
            return True

        return self.isSubPath(head, root.left) or self.isSubPath(head, root.right)

    def checkPath(self, head: ListNode, root: TreeNode) -> bool:
        if head is None:
            return True
        elif root is None or head.val != root.val:
            return False

        return self.checkPath(head.next, root.left) or self.checkPath(head.next, root.right)
```

## Solutions (Ruby)

### 1. Recursion
```Ruby
# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
# @param {ListNode} head
# @param {TreeNode} root
# @return {Boolean}
def is_sub_path(head, root)
  return false if root.nil?
  return true if head.val == root.val && check_path(head, root)

  is_sub_path(head, root.left) || is_sub_path(head, root.right)
end

# @param {ListNode} head
# @param {TreeNode} root
# @return {Boolean}
def check_path(head, root)
  return true if head.nil?
  return false if root.nil? || head.val != root.val

  check_path(head.next, root.left) || check_path(head.next, root.right)
end
```
