# 2181. Merge Nodes in Between Zeros
You are given the `head` of a linked list, which contains a series of integers **separated** by `0`'s. The **beginning** and **end** of the linked list will have `Node.val == 0`.

For **every** two consecutive `0`'s, **merge** all the nodes lying in between them into a single node whose value is the **sum** of all the merged nodes. The modified list should not contain any `0`'s.

Return *the* `head` *of the modified linked list*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/02/02/ex1-1.png)
<pre>
<strong>Input:</strong> head = [0,3,1,0,4,5,2,0]
<strong>Output:</strong> [4,11]
<strong>Explanation:</strong>
The above figure represents the given linked list. The modified list contains
- The sum of the nodes marked in green: 3 + 1 = 4.
- The sum of the nodes marked in red: 4 + 5 + 2 = 11.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/02/02/ex2-1.png)
<pre>
<strong>Input:</strong> head = [0,1,0,3,0,2,2,0]
<strong>Output:</strong> [1,3,4]
<strong>Explanation:</strong> The above figure represents the given linked list. The modified list contains
- The sum of the nodes marked in green: 1 = 1.
- The sum of the nodes marked in red: 3 = 3.
- The sum of the nodes marked in yellow: 2 + 2 = 4.
</pre>

#### Constraints:
* The number of nodes in the list is in the range <code>[3, 2 * 10<sup>5</sup>]</code>.
* `0 <= Node.val <= 1000`
* There are **no** two consecutive nodes with `Node.val == 0`.
* The **beginning** and **end** of the linked list have `Node.val == 0`.

## Solutions (Python)

### 1. Solution
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        merged = head
        curr = merged.next

        while curr.next:
            if curr.val != 0:
                merged.val += curr.val
                merged.next = curr.next
            else:
                merged = curr
            curr = merged.next

        merged.next = None

        return head
```
