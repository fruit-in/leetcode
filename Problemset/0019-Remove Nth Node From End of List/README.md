# 19. Remove Nth Node From End of List
Given a linked list, remove the *n*-th node from the end of list and return its head.

#### Example:
<pre>
Given linked list: <strong>1->2->3->4->5</strong>, and <strong><em>n</em> = 2</strong>.

After removing the second node from the end, the linked list becomes <strong>1->2->3->5</strong>.
</pre>

#### Note:
Given *n* will always be valid.

#### Follow up:
Could you do this in one pass?

## Solutions (Python)

### 1. Solution
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

## Solutions (Ruby)

### 1. Solution
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
