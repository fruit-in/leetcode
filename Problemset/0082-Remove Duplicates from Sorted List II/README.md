# 82. Remove Duplicates from Sorted List II
Given a sorted linked list, delete all nodes that have duplicate numbers, leaving only *distinct* numbers from the original list.

Return the linked list sorted as well.

#### Example 1:
<pre>
<strong>Input:</strong> 1->2->3->3->4->4->5
<strong>Output:</strong> 1->2->5
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 1->1->1->2->3
<strong>Output:</strong> 2->3
</pre>

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
# @return {ListNode}
def delete_duplicates(head)
  dummy = ListNode.new(0, head)
  curr = dummy

  until curr.nil?
    if !curr.next.nil? && !curr.next.next.nil? && curr.next.val == curr.next.next.val
      curr.next.next = curr.next.next.next while !curr.next.next.nil? && curr.next.val == curr.next.next.val
      curr.next = curr.next.next
    else
      curr = curr.next
    end
  end

  dummy.next
end
```
