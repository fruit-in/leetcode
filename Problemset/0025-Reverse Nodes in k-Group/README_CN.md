# 25. K 个一组翻转链表
给你链表的头节点 `head` ，每 `k` 个节点一组进行翻转，请你返回修改后的链表。

`k` 是一个正整数，它的值小于或等于链表的长度。如果节点总数不是 `k` 的整数倍，那么请将最后剩余的节点保持原有顺序。

你不能只是单纯的改变节点内部的值，而是需要实际进行节点交换。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/10/03/reverse_ex1.jpg)
<pre>
<strong>输入:</strong> head = [1,2,3,4,5], k = 2
<strong>输出:</strong> [2,1,4,3,5]
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/10/03/reverse_ex2.jpg)
<pre>
<strong>输入:</strong> head = [1,2,3,4,5], k = 3
<strong>输出:</strong> [3,2,1,4,5]
</pre>

#### 提示:
* 链表中的节点数目为 `n`
* `1 <= k <= n <= 5000`
* `0 <= Node.val <= 1000`

**进阶：**你可以设计一个只用 `O(1)` 额外内存空间的算法解决此问题吗？

## 题解 (Python)

### 1. 题解
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        length = 0
        curr = head
        while curr is not None:
            length += 1
            curr = curr.next

        dummy = ListNode(next=head)
        grouptail = dummy

        for _ in range(length // k):
            grouphead = grouptail.next
            prev = grouphead
            for _ in range(k):
                prev = prev.next

            curr = grouphead
            for _ in range(k):
                temp = curr
                curr = curr.next
                temp.next = prev
                prev = temp

            grouptail.next = prev
            grouptail = grouphead

        return dummy.next
```
