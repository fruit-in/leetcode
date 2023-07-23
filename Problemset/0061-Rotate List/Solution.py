# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def rotateRight(self, head: ListNode, k: int) -> ListNode:
        if not head:
            return None

        length = 1
        curr = head
        while curr.next:
            curr = curr.next
            length += 1
        curr.next = head

        for _ in range(length - k % length - 1):
            head = head.next

        new_head = head.next
        head.next = None

        return new_head
