# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def partition(self, head: ListNode, x: int) -> ListNode:
        head_lt = ListNode()
        head_ge = ListNode()
        curr_lt = head_lt
        curr_ge = head_ge

        while head:
            if head.val < x:
                curr_lt.next = head
                curr_lt = curr_lt.next
            else:
                curr_ge.next = head
                curr_ge = curr_ge.next

            head = head.next

        curr_lt.next = head_ge.next
        curr_ge.next = None

        return head_lt.next
