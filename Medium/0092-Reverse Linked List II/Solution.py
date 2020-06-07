# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseBetween(self, head: ListNode, m: int, n: int) -> ListNode:
        dummy = ListNode(next=head)
        start = curr = dummy
        next = curr.next

        for _ in range(m - 1):
            start = curr = next
            next = curr.next

        for _ in range(n - m + 1):
            prev = curr
            curr = next
            next = curr.next
            curr.next = prev

        start.next.next = next
        start.next = curr

        return dummy.next
