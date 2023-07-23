# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def splitListToParts(self, root: ListNode, k: int) -> List[ListNode]:
        curr = root
        length = 0
        while curr:
            length += 1
            curr = curr.next

        curr = root
        ret = []
        for i in range(k):
            ret.append(curr)
            for _ in range(length // k + (i < length % k) - 1):
                curr = curr.next
            if curr:
                curr.next, curr = None, curr.next

        return ret
