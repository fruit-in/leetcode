"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""


class Solution:
    def connect(self, root: 'Node') -> 'Node':
        parent = root

        while True:
            while parent is not None and parent.left is None and parent.right is None:
                parent = parent.next

            if parent is None:
                break

            if parent.left is not None:
                head = parent.left
                curr = head
                if parent.right is not None:
                    curr.next = parent.right
                    curr = curr.next
            else:
                head = parent.right
                curr = head

            parent = parent.next

            while True:
                while parent is not None and parent.left is None and parent.right is None:
                    parent = parent.next

                if parent is None:
                    break

                if parent.left is not None:
                    curr.next = parent.left
                    curr = curr.next
                if parent.right is not None:
                    curr.next = parent.right
                    curr = curr.next

                parent = parent.next

            parent = head

        return root
