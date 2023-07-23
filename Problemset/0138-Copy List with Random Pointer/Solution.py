"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""

class Solution:
    def copyRandomList(self, head: 'Node') -> 'Node':
        if not head:
            return None

        curr = head

        while curr:
            copy = Node(curr.val, curr.next, curr.random)
            curr.next = copy
            curr = copy.next

        curr = head.next

        while curr:
            curr.next = curr.next.next if curr.next else None
            curr.random = curr.random.next if curr.random else None
            curr = curr.next

        return head.next
