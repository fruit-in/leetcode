# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        merged = head
        curr = merged.next

        while curr.next:
            if curr.val != 0:
                merged.val += curr.val
                merged.next = curr.next
            else:
                merged = curr
            curr = merged.next

        merged.next = None

        return head
