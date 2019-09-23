# 141. 环形链表
给定一个链表，判断链表中是否有环。
为了表示给定链表中的环，我们使用整数 ```pos``` 来表示链表尾连接到链表中的位置（索引从 0 开始）。 如果 ```pos``` 是 ```-1```，则在该链表中没有环。

#### 示例 1:
<pre>
<strong>输入:</strong> head = [3,2,0,-4], pos = 1
<strong>输出:</strong> true
<strong>解释:</strong> 链表中有一个环，其尾部连接到第二个节点。
</pre>
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/07/circularlinkedlist.png)

#### 示例 2:
<pre>
<strong>输入:</strong> head = [1,2], pos = 0
<strong>输出:</strong> true
<strong>解释:</strong> 链表中有一个环，其尾部连接到第一个节点。
</pre>
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/07/circularlinkedlist_test2.png)

#### 示例 3:
<pre>
<strong>输入:</strong> head = [1], pos = -1
<strong>输出:</strong> false
<strong>解释:</strong> 链表中没有环。
</pre>
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/07/circularlinkedlist_test3.png)

#### 进阶:
你能用 *O(1)*（即，常量）内存解决此问题吗？

## 题解 (Python)

### 1. 集合
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def hasCycle(self, head: ListNode) -> bool:
        nodes = set()
        while head:
            if head in nodes:
                return True
            nodes.add(head)
            head = head.next
        return False
```

### 2. 双指针
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def hasCycle(self, head: ListNode) -> bool:
        slow, fast = head, head
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
            if slow == fast:
                return True
        return False
```
