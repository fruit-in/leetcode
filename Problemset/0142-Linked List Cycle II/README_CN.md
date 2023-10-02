# 142. 环形链表 II
给定一个链表的头节点  `head` ，返回链表开始入环的第一个节点。 *如果链表无环，则返回 `null`*。

如果链表中有某个节点，可以通过连续跟踪 `next` 指针再次到达，则链表中存在环。 为了表示给定链表中的环，评测系统内部使用整数 `pos` 来表示链表尾连接到链表中的位置（**索引从 0 开始**）。如果 `pos` 是 `-1`，则在该链表中没有环。**注意：**`pos` **不作为参数进行传递**，仅仅是为了标识链表的实际情况。

**不允许修改** 链表。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist.png)
<pre>
<strong>输入:</strong> head = [3,2,0,-4], pos = 1
<strong>输出:</strong> 返回索引为 1 的链表节点
<strong>解释:</strong> 链表中有一个环，其尾部连接到第二个节点。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist_test2.png)
<pre>
<strong>输入:</strong> head = [1,2], pos = 0
<strong>输出:</strong> 返回索引为 0 的链表节点
<strong>解释:</strong> 链表中有一个环，其尾部连接到第一个节点。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist_test3.png)
<pre>
<strong>输入:</strong> head = [1], pos = -1
<strong>输出:</strong> 返回 null
<strong>解释:</strong> 链表中没有环。
</pre>

#### 提示:
* 链表中节点的数目范围在范围 <code>[0, 10<sup>4</sup>]</code> 内
* <code>-10<sup>5</sup> <= Node.val <= 10<sup>5</sup></code>
* `pos` 的值为 `-1` 或者链表中的一个有效索引

**进阶：**你是否可以使用 `O(1)` 空间解决此题？

## 题解 (Python)

### 1. 题解
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None or head.next is None:
            return None

        slow = head.next
        fast = head.next.next

        while slow != fast:
            if fast is None or fast.next is None:
                return None

            slow = slow.next
            fast = fast.next.next

        while head != slow:
            head = head.next
            slow = slow.next

        return head
```
