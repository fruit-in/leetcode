# 61. 旋转链表
给定一个链表，旋转链表，将链表每个节点向右移动 *k* 个位置，其中 *k* 是非负数。

#### 示例 1:
<pre>
<strong>输入:</strong> 1->2->3->4->5->NULL, k = 2
<strong>输出:</strong> 4->5->1->2->3->NULL
<strong>解释:</strong>
向右旋转 1 步: 5->1->2->3->4->NULL
向右旋转 2 步: 4->5->1->2->3->NULL
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 0->1->2->NULL, k = 4
<strong>输出:</strong> 2->0->1->NULL
<strong>解释:</strong>
向右旋转 1 步: 2->0->1->NULL
向右旋转 2 步: 1->2->0->NULL
向右旋转 3 步: 0->1->2->NULL
向右旋转 4 步: 2->0->1->NULL
</pre>

## 题解 (Python)

### 1. 题解
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def rotateRight(self, head: ListNode, k: int) -> ListNode:
        if not head:
            return None

        length = 1
        curr = head
        while curr.next:
            curr = curr.next
            length += 1
        curr.next = head

        for _ in range(length - k % length - 1):
            head = head.next

        new_head = head.next
        head.next = None

        return new_head
```
