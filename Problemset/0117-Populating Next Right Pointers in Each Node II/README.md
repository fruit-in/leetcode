# 117. Populating Next Right Pointers in Each Node II
Given a binary tree

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

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/02/15/117_sample.png)
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,null,7]
<strong>Output:</strong> [1,#,2,3,#,4,5,7,#]
<strong>Explanation:</strong> Given the above binary tree (Figure A), your function should populate each next pointer to point to its next right node, just like in Figure B. The serialized output is in level order as connected by the next pointers, with '#' signifying the end of each level.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> root = []
<strong>Output:</strong> []
</pre>

#### Constraints:
* The number of nodes in the tree is in the range `[0, 6000]`.
* `-100 <= Node.val <= 100`

#### Follow-up:
* You may only use constant extra space.
* The recursive approach is fine. You may assume implicit stack space does not count as extra space for this problem.

## Solutions (Python)

### 1. Solution
```Python
"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""


class Solution:
    def connect(self, root: 'Node') -> 'Node':
        parent = root

        while True:
            while parent is not None and parent.left is None and parent.right is None:
                parent = parent.next

            if parent is None:
                break

            if parent.left is not None:
                head = parent.left
                curr = head
                if parent.right is not None:
                    curr.next = parent.right
                    curr = curr.next
            else:
                head = parent.right
                curr = head

            parent = parent.next

            while True:
                while parent is not None and parent.left is None and parent.right is None:
                    parent = parent.next

                if parent is None:
                    break

                if parent.left is not None:
                    curr.next = parent.left
                    curr = curr.next
                if parent.right is not None:
                    curr.next = parent.right
                    curr = curr.next

                parent = parent.next

            parent = head

        return root
```
