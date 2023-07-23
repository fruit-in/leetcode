# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        l3 = ListNode(0)
        curr = l3
        carry = 0

        while l1 or l2:
            curr.next = ListNode(carry)
            curr = curr.next
            curr.val += l1.val if l1 else 0
            curr.val += l2.val if l2 else 0
            carry = curr.val // 10
            curr.val %= 10

            l1 = l1.next if l1 else None
            l2 = l2.next if l2 else None

        if carry:
            curr.next = ListNode(carry)

        return l3.next
