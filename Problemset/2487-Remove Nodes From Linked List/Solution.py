# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is not None:
            head.next = self.removeNodes(head.next)
            if head.next is not None and head.val < head.next.val:
                head = head.next

        return head
