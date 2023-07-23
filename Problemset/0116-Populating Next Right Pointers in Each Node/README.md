# 116. Populating Next Right Pointers in Each Node
You are given a **perfect binary tree** where all leaves are on the same level, and every parent has two children. The binary tree has the following definition:
```
struct Node {
  int val;
  Node *left;
  Node *right;
  Node *next;
}
```

Populate each next pointer to point to its next right node. If there is no next right node, the next pointer should be set to `NULL`.

Initially, all next pointers are set to `NULL`.

#### Follow up:
* You may only use constant extra space.
* Recursive approach is fine, you may assume implicit stack space does not count as extra space for this problem.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/02/14/116_sample.png)
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,6,7]
<strong>Output:</strong> [1,#,2,3,#,4,5,6,7,#]
<strong>Explanation:</strong> Given the above perfect binary tree (Figure A), your function should populate each next pointer to point to its next right node, just like in Figure B. The serialized output is in level order as connected by the next pointers, with '#' signifying the end of each level.
</pre>

#### Constraints:
* The number of nodes in the given tree is less than `4096`.
* `-1000 <= node.val <= 1000`

## Solutions (Ruby)

### 1. Solution
```Ruby
# Definition for Node.
# class Node
#     attr_accessor :val, :left, :right, :next
#     def initialize(val)
#         @val = val
#         @left, @right, @next = nil, nil, nil
#     end
# end

# @param {Node} root
# @return {Node}
def connect(root)
    head = root

    while not head.nil?
        curr = head
        head = head.left

        while not curr.nil?
            if not curr.left.nil?
                curr.left.next = curr.right
                curr.right.next = curr.next.left if not curr.next.nil?
            end
            curr = curr.next
        end
    end

    return root
end
```
