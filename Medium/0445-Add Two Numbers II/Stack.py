# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        stack1 = []
        stack2 = []

        while l1:
            stack1.append(l1.val)
            l1 = l1.next
        while l2:
            stack2.append(l2.val)
            l2 = l2.next

        head = None
        carry = 0

        while stack1 or stack2 or carry:
            digit = carry
            digit += stack1.pop() if stack1 else 0
            digit += stack2.pop() if stack2 else 0
            carry = 1 if digit > 9 else 0
            digit -= 10 if digit > 9 else 0

            temp = ListNode(digit)
            temp.next = head
            head = temp

        return head
