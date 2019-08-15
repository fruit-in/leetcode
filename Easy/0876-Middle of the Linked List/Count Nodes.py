# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def middleNode(self, head: ListNode) -> ListNode:
        list_len = 0
        p = head
        while p:
            list_len += 1
            p = p.next
        for i in range(list_len // 2):
            head = head.next
        return head
