# 92. Reverse Linked List II
Reverse a linked list from position *m* to *n*. Do it in one-pass.

**Note:** 1 ≤ *m* ≤ *n* ≤ length of list.

#### Example:
<pre>
<strong>Input:</strong> 1->2->3->4->5->NULL, <em>m</em> = 2, <em>n</em> = 4
<strong>Output:</strong> 1->4->3->2->5->NULL
</pre>

## Solutions (Python)

### 1. Solution
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseBetween(self, head: ListNode, m: int, n: int) -> ListNode:
        dummy = ListNode(next=head)
        start = curr = dummy
        next = curr.next

        for _ in range(m - 1):
            start = curr = next
            next = curr.next

        for _ in range(n - m + 1):
            prev = curr
            curr = next
            next = curr.next
            curr.next = prev

        start.next.next = next
        start.next = curr

        return dummy.next
```
