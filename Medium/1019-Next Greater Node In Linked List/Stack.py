# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def nextLargerNodes(self, head: ListNode) -> List[int]:
        curr = head
        vals = []

        while curr is not None:
            vals.append(curr.val)
            curr = curr.next

        stack = []
        ret = [0] * len(vals)

        for i in range(len(vals) - 1, -1, -1):
            while stack != [] and vals[i] >= stack[-1]:
                stack.pop()
            if stack != []:
                ret[i] = stack[-1]
            stack.append(vals[i])

        return ret
