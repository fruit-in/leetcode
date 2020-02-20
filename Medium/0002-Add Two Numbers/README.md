# 2. Add Two Numbers
You are given two **non-empty** linked lists representing two non-negative integers. The digits are stored in **reverse order** and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

#### Example:
<pre>
<strong>Input:</strong> (2 -> 4 -> 3) + (5 -> 6 -> 4)
<strong>Output:</strong> 7 -> 0 -> 8
<strong>Explanation:</strong> 342 + 465 = 807.
</pre>

## Solutions (Python)

### 1. Solution
```Python
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
```
