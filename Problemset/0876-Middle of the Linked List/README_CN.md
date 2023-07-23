# 876. 链表的中间结点
给定一个带有头结点 ```head``` 的非空单链表，返回链表的中间结点。

如果有两个中间结点，则返回第二个中间结点。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,3,4,5]
<strong>输出:</strong> 此列表中的结点 3 (序列化形式：[3,4,5])
返回的结点值为 3 。 (测评系统对该结点序列化表述是 [3,4,5])。
注意，我们返回了一个 ListNode 类型的对象 ans，这样：
ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, 以及 ans.next.next.next = NULL.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,2,3,4,5,6]
<strong>输出:</strong> 此列表中的结点 4 (序列化形式：[4,5,6])
由于该列表有两个中间结点，值分别为 3 和 4，我们返回第二个结点。
</pre>

#### 提示:
* 给定链表的结点数介于 ```1``` 和 ```100``` 之间。

## 题解 (Python)

### 1. 双指针
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def middleNode(self, head: ListNode) -> ListNode:
        p1 = head
        p2 = head
        while p2 and p2.next:
            p1 = p1.next
            p2 = p2.next.next
        return p1
```

### 2. 计数
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def middleNode(self, head: ListNode) -> ListNode:
        list_len = 0
        p = head
        while p:
            list_len += 1
            p = p.next
        for i in range(list_len // 2):
            head = head.next
        return head
```
