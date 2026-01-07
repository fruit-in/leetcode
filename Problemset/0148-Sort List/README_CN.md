# 148. 排序链表
给你链表的头结点 `head` ，请将其按 **升序** 排列并返回 **排序后的链表** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/14/sort_list_1.jpg)
<pre>
<strong>输入:</strong> head = [4,2,1,3]
<strong>输出:</strong> [1,2,3,4]
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/09/14/sort_list_2.jpg)
<pre>
<strong>输入:</strong> head = [-1,5,3,4,0]
<strong>输出:</strong> [-1,0,3,4,5]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> head = []
<strong>输出:</strong> []
</pre>

#### 提示:
* 链表中节点的数目在范围 <code>[0, 5 * 10<sup>4</sup>]</code> 内
* <code>-10<sup>5</sup> <= Node.val <= 10<sup>5</sup></code>

**进阶：**你可以在 `O(n log n)` 时间复杂度和常数级空间复杂度下，对链表进行排序吗？

## 题解 (Python)

### 1. 题解
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def sortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        def forward(head: Optional[ListNode], step: int) -> Optional[ListNode]:
            i = 0

            while i < step and head:
                head = head.next
                i += 1

            return head

        def merge(head1: Optional[ListNode], len1: int, head2: Optional[ListNode], len2: int) -> (Optional[ListNode], Optional[ListNode]):
            i, j = 0, 0
            tail = hair = ListNode()

            while i < len1 or j < len2:
                if j >= len2 or (i < len1 and head1.val <= head2.val):
                    tail.next = head1
                    head1 = head1.next
                    i += 1
                else:
                    tail.next = head2
                    head2 = head2.next
                    j += 1
                tail = tail.next

            tail.next = None

            return (hair.next, tail)

        hair = ListNode(0, head)
        length = 0
        size = 1

        while head:
            length += 1
            head = head.next

        while size < length:
            newhead1 = hair.next
            tail = hair

            for i in range(0, length, size * 2):
                head1 = newhead1
                head2 = forward(head1, size)
                newhead1 = forward(head2, size)
                len1 = min(length - i, size)
                len2 = min(max(length - i - size, 0), size)
                tail.next, tail = merge(head1, len1, head2, len2)

            size *= 2

        return hair.next
```
