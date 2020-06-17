# 86. Partition List
Given a linked list and a value *x*, partition it such that all nodes less than *x* come before nodes greater than or equal to *x*.

You should preserve the original relative order of the nodes in each of the two partitions.

#### Example:
<pre>
<strong>Input:</strong> head = 1->4->3->2->5->2, x = 3
<strong>Output:</strong> 1->2->2->4->3->5
</pre>

## Solutions (Python)

### 1. Two Pointers
```Python
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
```
