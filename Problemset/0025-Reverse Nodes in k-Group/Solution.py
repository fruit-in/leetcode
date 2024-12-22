# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        length = 0
        curr = head
        while curr is not None:
            length += 1
            curr = curr.next

        dummy = ListNode(next=head)
        grouptail = dummy

        for _ in range(length // k):
            grouphead = grouptail.next
            prev = grouphead
            for _ in range(k):
                prev = prev.next

            curr = grouphead
            for _ in range(k):
                temp = curr
                curr = curr.next
                temp.next = prev
                prev = temp

            grouptail.next = prev
            grouptail = grouphead

        return dummy.next
