# 1290. Convert Binary Number in a Linked List to Integer
Given ```head``` which is a reference node to a singly-linked list. The value of each node in the linked list is either 0 or 1. The linked list holds the binary representation of a number.

Return the *decimal value* of the number in the linked list.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/12/05/graph-1.png)
<pre>
<strong>Input:</strong> head = [1,0,1]
<strong>Output:</strong> 5
<strong>Explanation:</strong> (101) in base 2 = (5) in base 10
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> head = [0]
<strong>Output:</strong> 0
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> head = [1]
<strong>Output:</strong> 1
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> head = [1,0,0,1,0,0,1,1,1,0,0,0,0,0,0]
<strong>Output:</strong> 18880
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> head = [0,0]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* The Linked List is not empty.
* Number of nodes will not exceed ```30```.
* Each node's value is either ```0``` or ```1```.

## Solutions (Python)

### 1. Solution
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def getDecimalValue(self, head: ListNode) -> int:
        ret = 0

        while head:
            ret <<= 1
            ret += head.val
            head = head.next

        return ret
```
