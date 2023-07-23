# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def isPalindrome(self, head: ListNode) -> bool:
        if not head:
            return True
        mid, end = head, head.next
        while end and end.next:
            mid = mid.next
            end = end.next.next

        prev = mid
        curr = mid.next
        while curr:
            next = curr.next
            curr.next = prev
            prev = curr
            curr = next

        while head != mid.next:
            if head.val != prev.val:
                return False
            head = head.next
            prev = prev.next
        return True
