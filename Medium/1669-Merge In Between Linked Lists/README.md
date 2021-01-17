# 1669. Merge In Between Linked Lists
You are given two linked lists: `list1` and `list2` of sizes `n` and `m` respectively.

Remove `list1`'s nodes from the <code>a<sup>th</sup></code> node to the <code>b<sup>th</sup></code> node, and put `list2` in their place.

The blue edges and nodes in the following figure incidate the result:<br>
![](https://assets.leetcode.com/uploads/2020/11/05/fig1.png)<br>
*Build the result list and return its head.*

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/11/05/merge_linked_list_ex1.png)
<pre>
<strong>Input:</strong> list1 = [0,1,2,3,4,5], a = 3, b = 4, list2 = [1000000,1000001,1000002]
<strong>Output:</strong> [0,1,2,1000000,1000001,1000002,5]
<strong>Explanation:</strong> We remove the nodes 3 and 4 and put the entire list2 in their place. The blue edges and nodes in the above figure indicate the result.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/11/05/merge_linked_list_ex2.png)
<pre>
<strong>Input:</strong> list1 = [0,1,2,3,4,5,6], a = 2, b = 5, list2 = [1000000,1000001,1000002,1000003,1000004]
<strong>Output:</strong> [0,1,1000000,1000001,1000002,1000003,1000004,6]
<strong>Explanation:</strong> The blue edges and nodes in the above figure indicate the result.
</pre>

#### Constraints:
* <code>3 <= list1.length <= 10<sup>4</sup></code>
* `1 <= a <= b < list1.length - 1`
* <code>1 <= list2.length <= 10<sup>4</sup></code>

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
# @param {ListNode} list1
# @param {Integer} a
# @param {Integer} b
# @param {ListNode} list2
# @return {ListNode}
def merge_in_between(list1, a, b, list2)
  removed_a = list1
  (1...a).each do |_|
    removed_a = removed_a.next
  end
  removed_b = removed_a.next
  (0..(b - a)).each do |_|
    removed_b = removed_b.next
  end

  curr = list2
  curr = curr.next until curr.next.nil?

  removed_a.next = list2
  curr.next = removed_b

  list1
end
```
