# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def isPalindrome(self, head: ListNode) -> bool:
        if not head:
            return True

        head_ = ListNode(head.val)
        curr = head
        while curr.next:
            curr = curr.next
            head_new = ListNode(curr.val)
            head_new.next = head_
            head_ = head_new

        while head:
            if head.val != head_.val:
                return False
            head = head.next
            head_ = head_.next
        return True
