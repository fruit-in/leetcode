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
