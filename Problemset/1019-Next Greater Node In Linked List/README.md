# 1019. Next Greater Node In Linked List
We are given a linked list with `head` as the first node.  Let's number the nodes in the list: `node_1, node_2, node_3, ...` etc.

Each node may have a *next larger* **value**: for `node_i`, `next_larger(node_i)` is the `node_j.val` such that `j > i`, `node_j.val > node_i.val`, and `j` is the smallest possible choice.  If such a `j` does not exist, the next larger value is `0`.

Return an array of integers `answer`, where `answer[i] = next_larger(node_{i+1})`.

Note that in the example **inputs** (not outputs) below, arrays such as `[2,1,5]` represent the serialization of a linked list with a head node value of 2, second node value of 1, and third node value of 5.

#### Example 1:
<pre>
<strong>Input:</strong> [2,1,5]
<strong>Output:</strong> [5,5,0]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [2,7,4,3,5]
<strong>Output:</strong> [7,0,5,5,0]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [1,7,5,1,9,2,5,1]
<strong>Output:</strong> [7,9,9,9,0,5,0,0]
</pre>

#### Note:
1. `1 <= node.val <= 10^9` for each node in the linked list.
2. The given list has length in the range `[0, 10000]`.

## Solutions (Python)

### 1. Stack
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def nextLargerNodes(self, head: ListNode) -> List[int]:
        curr = head
        vals = []

        while curr is not None:
            vals.append(curr.val)
            curr = curr.next

        stack = []
        ret = [0] * len(vals)

        for i in range(len(vals) - 1, -1, -1):
            while stack != [] and vals[i] >= stack[-1]:
                stack.pop()
            if stack != []:
                ret[i] = stack[-1]
            stack.append(vals[i])

        return ret
```

## Solutions (Ruby)

### 1. Stack
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
# @return {Integer[]}
def next_larger_nodes(head)
  curr = head
  vals = []

  until curr.nil?
    vals.push(curr.val)
    curr = curr.next
  end

  stack = []
  ret = [0] * vals.size

  (vals.size - 1..0).step(-1).each do |i|
    stack.pop until stack.empty? || vals[i] < stack[-1]
    ret[i] = stack[-1] unless stack.empty?
    stack.push(vals[i])
  end

  ret
end
```
