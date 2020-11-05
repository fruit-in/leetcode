# 430. Flatten a Multilevel Doubly Linked List
You are given a doubly linked list which in addition to the next and previous pointers, it could have a child pointer, which may or may not point to a separate doubly linked list. These child lists may have one or more children of their own, and so on, to produce a multilevel data structure, as shown in the example below.

Flatten the list so that all the nodes appear in a single-level, doubly linked list. You are given the head of the first level of the list.

#### Example 1:
<pre>
<b>Input:</b> head = [1,2,3,4,5,6,null,null,null,7,8,9,10,null,null,11,12]
<b>Output:</b> [1,2,3,7,8,11,12,9,10,4,5,6]
<b>Explanation:</b>
The multilevel linked list in the input is as follows:
<img src="https://assets.leetcode.com/uploads/2018/10/12/multilevellinkedlist.png">
After flattening the multilevel linked list it becomes:
<img src="https://assets.leetcode.com/uploads/2018/10/12/multilevellinkedlistflattened.png">
</pre>

#### Example 2:
<pre>
<b>Input:</b> head = [1,2,null,3]
<b>Output:</b> [1,3,2]
<b>Explanation:</b>
The input multilevel linked list is as follows:

  1---2---NULL
  |
  3---NULL
</pre>

#### Example 3:
<pre>
<b>Input:</b> head = []
<b>Output:</b> []
</pre>

#### How multilevel linked list is represented in test case:
We use the multilevel linked list from **Example 1** above:
```
 1---2---3---4---5---6--NULL
         |
         7---8---9---10--NULL
             |
             11--12--NULL
```

The serialization of each level is as follows:
```
[1,2,3,4,5,6,null]
[7,8,9,10,null]
[11,12,null]
```

To serialize all levels together we will add nulls in each level to signify no node connects to the upper node of the previous level. The serialization becomes:

```
[1,2,3,4,5,6,null]
[null,null,7,8,9,10,null]
[null,11,12,null]
```

Merging the serialization of each level and removing trailing nulls we obtain:
```
[1,2,3,4,5,6,null,null,null,7,8,9,10,null,null,11,12]
```

#### Constraints:
* Number of Nodes will not exceed 1000.
* `1 <= Node.val <= 10^5`

## Solutions (Ruby)

### 1. DFS
```Ruby
# Definition for a Node.
# class Node
#     attr_accessor :val, :prev, :next, :child
#     def initialize(val=nil, prev=nil, next_=nil, child=nil)
#         @val = val
#         @prev = prev
#         @next = next_
#         @child = child
#     end
# end

# @param {Node} root
# @return {Node}
def flatten(root)
    return nil if root.nil?

    prev = Node.new
    stack = [root]

    until stack.empty?
        curr = stack.pop

        stack.push(curr.next) unless curr.next.nil?
        stack.push(curr.child) unless curr.child.nil?

        prev.next = curr
        curr.prev = prev
        curr.child = nil

        prev = curr
    end

    root.prev = nil

    return root
end
```
