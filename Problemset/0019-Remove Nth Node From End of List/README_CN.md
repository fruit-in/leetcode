# 19. 删除链表的倒数第N个节点
给定一个链表，删除链表的倒数第 *n* 个节点，并且返回链表的头结点。

#### 示例:
<pre>
给定一个链表: <strong>1->2->3->4->5</strong>, 和 <strong><em>n</em> = 2</strong>.

当删除了倒数第二个节点后，链表变为 <strong>1->2->3->5</strong>.
</pre>

#### 说明:
给定的 *n* 保证是有效的。

#### 进阶:
你能尝试使用一趟扫描实现吗？

## 题解 (Python)

### 1. 题解
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def removeNthFromEnd(self, head: ListNode, n: int) -> ListNode:
        dummy = ListNode(0)
        dummy.next = head
        node0 = head
        node1 = dummy

        for _ in range(n):
            node0 = node0.next

        while node0:
            node0 = node0.next
            node1 = node1.next

        node1.next = node1.next.next

        return dummy.next
```

## 题解 (Ruby)

### 1. 题解
```Ruby
# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @param {Integer} n
# @return {ListNode}
def remove_nth_from_end(head, n)
    dummy = ListNode.new(0, head)
    node0 = head
    node1 = dummy

    for _ in 0...n
        node0 = node0.next
    end

    while node0
        node0 = node0.next
        node1 = node1.next
    end

    node1.next = node1.next.next

    return dummy.next
end
```
