# 138. Copy List with Random Pointer
A linked list is given such that each node contains an additional random pointer which could point to any node in the list or null.

Return a **[deep copy](https://en.wikipedia.org/wiki/Object_copying#Deep_copy)** of the list.

The Linked List is represented in the input/output as a list of `n` nodes. Each node is represented as a pair of `[val, random_index]` where:
* `val`: an integer representing `Node.val`
* `random_index`: the index of the node (range from `0` to `n-1`) where random pointer points to, or `null` if it does not point to any node.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/12/18/e1.png)
<pre>
<strong>Input:</strong> head = [[7,null],[13,0],[11,4],[10,2],[1,0]]
<strong>Output:</strong> [[7,null],[13,0],[11,4],[10,2],[1,0]]
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/12/18/e2.png)
<pre>
<strong>Input:</strong> head = [[1,1],[2,1]]
<strong>Output:</strong> [[1,1],[2,1]]
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2019/12/18/e3.png)
<pre>
<strong>Input:</strong> head = [[3,null],[3,0],[3,null]]
<strong>Output:</strong> [[3,null],[3,0],[3,null]]
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> head = []
<strong>Output:</strong> []
<strong>Explanation:</strong> Given linked list is empty (null pointer), so return null.
</pre>

#### Constraints:
* `-10000 <= Node.val <= 10000`
* `Node.random` is null or pointing to a node in the linked list.
* Number of Nodes will not exceed 1000.

## Solutions (Python)

### 1. Solution
```Python
"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""

class Solution:
    def copyRandomList(self, head: 'Node') -> 'Node':
        if not head:
            return None

        curr = head

        while curr:
            copy = Node(curr.val, curr.next, curr.random)
            curr.next = copy
            curr = copy.next

        curr = head.next

        while curr:
            curr.next = curr.next.next if curr.next else None
            curr.random = curr.random.next if curr.random else None
            curr = curr.next

        return head.next
```
