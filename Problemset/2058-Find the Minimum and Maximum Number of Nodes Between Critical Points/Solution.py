# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def nodesBetweenCriticalPoints(self, head: Optional[ListNode]) -> List[int]:
        prev = head
        curr = head.next
        firstlocal = -1
        prevlocal = -1
        i = 0
        ret = [-1, -1]

        while curr.next:
            if (prev.val < curr.val and curr.val > curr.next.val) \
                    or (prev.val > curr.val and curr.val < curr.next.val):
                if firstlocal == -1:
                    firstlocal = i
                elif ret[0] == -1:
                    ret = [i - prevlocal, i - firstlocal]
                else:
                    ret = [min(ret[0], i - prevlocal), i - firstlocal]
                prevlocal = i

            prev = curr
            curr = curr.next
            i += 1

        return ret
