# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def deleteMiddle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head.next is None:
            return None

        n = 0
        curr = head

        while curr is not None:
            n += 1
            curr = curr.next

        x = n // 2
        curr = ListNode(next=head)

        for _ in range(x):
            curr = curr.next

        curr.next = curr.next.next

        return head
