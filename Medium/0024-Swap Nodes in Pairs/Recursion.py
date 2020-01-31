# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def swapPairs(self, head: ListNode) -> ListNode:
        if head and head.next:
            tmp = head.next.next
            head.next.next = head
            head = head.next
            head.next.next = self.swapPairs(tmp)

        return head
