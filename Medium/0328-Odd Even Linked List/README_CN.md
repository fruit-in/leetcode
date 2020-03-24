# 328. 奇偶链表
给定一个单链表，把所有的奇数节点和偶数节点分别排在一起。请注意，这里的奇数节点和偶数节点指的是节点编号的奇偶性，而不是节点的值的奇偶性。

请尝试使用原地算法完成。你的算法的空间复杂度应为 O(1)，时间复杂度应为 O(nodes)，nodes 为节点总数。

#### 示例 1:
<pre>
<strong>输入:</strong> 1->2->3->4->5->NULL
<strong>输出:</strong> 1->3->5->2->4->NULL
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 2->1->3->5->6->4->7->NULL
<strong>输出:</strong> 2->3->6->7->1->5->4->NULL
</pre>

#### 说明:
* 应当保持奇数节点和偶数节点的相对顺序。
* 链表的第一个节点视为奇数节点，第二个节点视为偶数节点，以此类推。

## 题解 (Python)

### 1. 题解
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def oddEvenList(self, head: ListNode) -> ListNode:
        if not head:
            return None

        even_head = head.next
        curr = head
        is_odd = True

        while curr.next:
            tmp = curr.next
            curr.next = tmp.next
            prev = curr
            curr = tmp
            is_odd = not is_odd

        if is_odd:
            curr.next = even_head
        else:
            prev.next = even_head

        return head
```
