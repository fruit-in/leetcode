# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def removeNthFromEnd(self, head: ListNode, n: int) -> ListNode:
        dummy = ListNode(0)
        dummy.next = head
        node0 = head
        node1 = dummy

        for _ in range(n):
            node0 = node0.next

        while node0:
            node0 = node0.next
            node1 = node1.next

        node1.next = node1.next.next

        return dummy.next
