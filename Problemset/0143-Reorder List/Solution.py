# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        """
        Do not return anything, modify head in-place instead.
        """
        length = 0
        curr = head
        while curr is not None:
            length += 1
            curr = curr.next

        prev = None
        curr = head
        for _ in range((length + 1) // 2):
            prev = curr
            curr = curr.next

        prev.next = None
        prev = None
        for _ in range(length // 2):
            temp = curr
            curr = curr.next
            temp.next = prev
            prev = temp

        curr0 = head
        curr1 = prev
        for _ in range(length // 2):
            prev0 = curr0
            prev1 = curr1
            curr0 = curr0.next
            curr1 = curr1.next
            prev0.next = prev1
            prev1.next = curr0
