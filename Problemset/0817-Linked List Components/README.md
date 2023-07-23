# 817. Linked List Components
We are given `head`, the head node of a linked list containing **unique integer values**.

We are also given the list `G`, a subset of the values in the linked list.

Return the number of connected components in `G`, where two values are connected if they appear consecutively in the linked list.

#### Example 1:
<pre>
<strong>Input:</strong>
head: 0->1->2->3
G = [0, 1, 3]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
0 and 1 are connected, so [0, 1] and [3] are the two connected components.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
head: 0->1->2->3->4
G = [0, 3, 1, 4]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
0 and 1 are connected, 3 and 4 are connected, so [0, 1] and [3, 4] are the two connected components.
</pre>

#### Note:
* If `N` is the length of the linked list given by `head`, `1 <= N <= 10000`.
* The value of each node in the linked list will be in the range `[0, N - 1]`.
* `1 <= G.length <= 10000`.
* `G` is a subset of all values in the linked list.

## Solutions (Python)

### 1. Solution
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def numComponents(self, head: ListNode, G: List[int]) -> int:
        G = set(G)
        curr = head
        ret = 0

        while curr:
            if curr.val in G and (not curr.next or curr.next.val not in G):
                ret += 1

            curr = curr.next

        return ret
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
# @param {Integer[]} g
# @return {Integer}
def num_components(head, g)
  g = g.to_set
  curr = head
  ret = 0

  until curr.nil?
    ret += 1 if g.member?(curr.val) && (curr.next.nil? || !g.member?(curr.next.val))

    curr = curr.next
  end

  ret
end
```
