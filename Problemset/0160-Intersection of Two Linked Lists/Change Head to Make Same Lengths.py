# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution(object):
    def getIntersectionNode(self, headA, headB):
        """
        :type head1, head1: ListNode
        :rtype: ListNode
        """
        cnt = 0
        curr = headA
        while curr:
            cnt += 1
            curr = curr.next

        curr = headB
        while curr:
            cnt -= 1
            curr = curr.next

        for _ in range(cnt):
            headA = headA.next
        for _ in range(-cnt):
            headB = headB.next

        while headA != headB:
            headA = headA.next
            headB = headB.next

        return headA
