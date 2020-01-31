# 24. 两两交换链表中的节点
给定一个链表，两两交换其中相邻的节点，并返回交换后的链表。

**你不能只是单纯的改变节点内部的值**，而是需要实际的进行节点交换。

#### 示例:
```
给定 1->2->3->4, 你应该返回 2->1->4->3.
```

## 题解 (Python)

### 1. 递归
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def swapPairs(self, head: ListNode) -> ListNode:
        if head and head.next:
            tmp = head.next.next
            head.next.next = head
            head = head.next
            head.next.next = self.swapPairs(tmp)

        return head
```
