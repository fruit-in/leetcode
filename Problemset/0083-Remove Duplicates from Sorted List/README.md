# 83. Remove Duplicates from Sorted List
Given a sorted linked list, delete all duplicates such that each element appear only *once*.

#### Example 1:
<pre>
<strong>Input:</strong> 1->1->2
<strong>Output:</strong> 1->2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 1->1->2->3->3
<strong>Output:</strong> 1->2->3
</pre>

## Solutions (Python)

### 1. Solution
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def deleteDuplicates(self, head: ListNode) -> ListNode:
        cur = head
        while cur and cur.next:
            if cur.val == cur.next.val:
                cur.next = cur.next.next
            else:
                cur = cur.next
        return head
```
