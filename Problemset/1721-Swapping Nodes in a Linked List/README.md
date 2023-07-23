# 1721. Swapping Nodes in a Linked List
You are given the `head` of a linked list, and an integer `k`.

Return *the head of the linked list after **swapping** the values of the* <code>k<sup>th</sup></code> *node from the beginning and the* <code>k<sup>th</sup></code> *node from the end (the list is **1-indexed**)*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/21/linked1.jpg)
<pre>
<strong>Input:</strong> head = [1,2,3,4,5], k = 2
<strong>Output:</strong> [1,4,3,2,5]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> head = [7,9,6,6,7,8,3,0,9,5], k = 5
<strong>Output:</strong> [7,9,6,6,8,7,3,0,9,5]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> head = [1], k = 1
<strong>Output:</strong> [1]
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> head = [1,2], k = 1
<strong>Output:</strong> [2,1]
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> head = [1,2,3], k = 2
<strong>Output:</strong> [1,2,3]
</pre>

#### Constraints:
* The number of nodes in the list is `n`.
* <code>1 <= k <= n <= 10<sup>5</sup></code>
* `0 <= Node.val <= 100`

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
# @param {Integer} k
# @return {ListNode}
def swap_nodes(head, k)
  dummy = ListNode.new(0, head)
  curr = head
  n = 0
  until curr.nil?
    n += 1
    curr = curr.next
  end

  curr = dummy
  node0 = nil
  node1 = nil
  (0..n).each do |i|
    node0 = curr if i == k - 1
    node1 = curr if i == n - k
    curr = curr.next
  end

  temp = node0.next
  node0.next = node1.next
  node1.next = temp
  temp = node0.next.next
  node0.next.next = node1.next.next
  node1.next.next = temp

  dummy.next
end
```
