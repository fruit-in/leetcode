# 92. 反转链表 II
反转从位置 *m* 到 *n* 的链表。请使用一趟扫描完成反转。

#### 说明:
1 ≤ *m* ≤ *n* ≤ 链表长度。

#### 示例:
<pre>
<strong>输入:</strong> 1->2->3->4->5->NULL, <em>m</em> = 2, <em>n</em> = 4
<strong>输出:</strong> 1->4->3->2->5->NULL
</pre>

## 题解 (Python)

### 1. 题解
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
