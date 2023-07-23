# 206. 反转链表
反转一个单链表。

#### 示例:
<pre>
<strong>输入:</strong> 1->2->3->4->5->NULL
<strong>输出:</strong> 5->4->3->2->1->NULL
</pre>

#### 进阶:
你可以迭代或递归地反转链表。你能否用两种方法解决这道题？

## 题解 (Python)

### 1. 递归
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def reverseList(self, head: ListNode) -> ListNode:
        if not head or not head.next:
            return head
        new_head = self.reverseList(head.next)
        head.next.next = head
        head.next = None
        return new_head
```

### 2. 迭代
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def reverseList(self, head: ListNode) -> ListNode:
        curr = head
        prev = None
        while curr:
            next = curr.next
            curr.next = prev
            prev = curr
            curr = next
        return prev
```
