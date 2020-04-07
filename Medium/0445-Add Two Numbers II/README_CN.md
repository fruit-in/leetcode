# 445. 两数相加 II
给定两个**非空**链表来代表两个非负整数。数字最高位位于链表开始位置。它们的每个节点只存储单个数字。将这两数相加会返回一个新的链表。

你可以假设除了数字 0 之外，这两个数字都不会以零开头。

#### 进阶:
如果输入链表不能修改该如何处理？换句话说，你不能对列表中的节点进行翻转。

#### 示例:
<pre>
<strong>输入:</strong> (7 -> 2 -> 4 -> 3) + (5 -> 6 -> 4)
<strong>输出:</strong> 7 -> 8 -> 0 -> 7
</pre>

## 题解 (Python)

### 1. 栈
```Python
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
```
