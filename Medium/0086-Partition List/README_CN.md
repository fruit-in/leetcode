# 86. 分隔链表
给定一个链表和一个特定值 *x*，对链表进行分隔，使得所有小于 *x* 的节点都在大于或等于 *x* 的节点之前。

你应当保留两个分区中每个节点的初始相对位置。

#### 示例:
<pre>
<strong>输入:</strong> head = 1->4->3->2->5->2, x = 3
<strong>输出:</strong> 1->2->2->4->3->5
</pre>

## 题解 (Python)

### 1. 双指针
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
