# 83. 删除排序链表中的重复元素
给定一个排序链表，删除所有重复的元素，使得每个元素只出现一次。

#### 示例 1:
<pre>
<strong>输入:</strong> 1->1->2
<strong>输出:</strong> 1->2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 1->1->2->3->3
<strong>输出:</strong> 1->2->3
</pre>

## 题解 (Python)

### 1. 题解
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
