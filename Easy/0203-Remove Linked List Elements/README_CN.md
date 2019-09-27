# 203. 移除链表元素
删除链表中等于给定值 ***val*** 的所有节点。

#### 示例:
<pre>
<strong>输入:</strong> 1->2->6->3->4->5->6, <strong><em>val</em></strong> = 6
<strong>输出:</strong> 1->2->3->4->5
</pre>

## 题解 (Python)

### 1. 双指针
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def removeElements(self, head: ListNode, val: int) -> ListNode:
        pre, cur = None, head
        while cur:
            if cur.val == val:
                if pre:
                    pre.next = cur.next
                else:
                    head = cur.next
            else:
                pre = cur
            cur = cur.next
        return head
```

### 2. 添加一个新表头
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def removeElements(self, head: ListNode, val: int) -> ListNode:
        cur = ListNode(0)
        cur.next = head
        head = cur
        while cur and cur.next:
            if cur.next.val == val:
                cur.next = cur.next.next
            else:
                cur = cur.next
        return head.next
```

### 3. 递归
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def removeElements(self, head: ListNode, val: int) -> ListNode:
        if not head:
            return None
        elif head.val == val:
            return self.removeElements(head.next, val)
        else:
            head.next = self.removeElements(head.next, val)
            return head
```
