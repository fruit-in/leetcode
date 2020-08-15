# 2. 两数相加
给出两个 **非空** 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 **逆序** 的方式存储的，并且它们的每个节点只能存储 **一位** 数字。

如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。

您可以假设除了数字 0 之外，这两个数都不会以 0 开头。

#### 示例:
<pre>
<strong>输入:</strong> (2 -> 4 -> 3) + (5 -> 6 -> 4)
<strong>输出:</strong> 7 -> 0 -> 8
<strong>原因:</strong> 342 + 465 = 807.
</pre>

## 题解 (Python)

### 1. 题解
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        l3 = ListNode(0)
        curr = l3
        carry = 0

        while l1 or l2:
            curr.next = ListNode(carry)
            curr = curr.next
            curr.val += l1.val if l1 else 0
            curr.val += l2.val if l2 else 0
            carry = curr.val // 10
            curr.val %= 10

            l1 = l1.next if l1 else None
            l2 = l2.next if l2 else None

        if carry:
            curr.next = ListNode(carry)

        return l3.next
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
# @param {ListNode} l1
# @param {ListNode} l2
# @return {ListNode}
def add_two_numbers(l1, l2)
    l3 = ListNode.new
    curr = l3
    carry = 0

    while l1 or l2
        curr.next = ListNode.new(carry)
        curr = curr.next
        curr.val += l1.val if l1
        curr.val += l2.val if l2
        carry = curr.val / 10
        curr.val %= 10

        l1 = l1.next if l1
        l2 = l2.next if l2
    end

    curr.next = ListNode.new(carry) if carry == 1

    return l3.next
end
```
