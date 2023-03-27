# 2058. Find the Minimum and Maximum Number of Nodes Between Critical Points
A **critical point** in a linked list is defined as **either** a **local maxima** or a **local minima**.

A node is a **local maxima** if the current node has a value **strictly greater** than the previous node and the next node.

A node is a **local minima** if the current node has a value **strictly smaller** than the previous node and the next node.

Note that a node can only be a local maxima/minima if there exists **both** a previous node and a next node.

Given a linked list `head`, return *an array of length 2 containing* `[minDistance, maxDistance]` *where* `minDistance` *is the **minimum distance** between **any two distinct** critical points and* `maxDistance` *is the **maximum distance** between **any two distinct** critical points. If there are **fewer** than two critical points, return* `[-1, -1]`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/10/13/a1.png)
<pre>
<strong>Input:</strong> head = [3,1]
<strong>Output:</strong> [-1,-1]
<strong>Explanation:</strong> There are no critical points in [3,1].
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/10/13/a2.png)
<pre>
<strong>Input:</strong> head = [5,3,1,2,5,1,2]
<strong>Output:</strong> [1,3]
<strong>Explanation:</strong> There are three critical points:
- [5,3,1,2,5,1,2]: The third node is a local minima because 1 is less than 3 and 2.
- [5,3,1,2,5,1,2]: The fifth node is a local maxima because 5 is greater than 2 and 1.
- [5,3,1,2,5,1,2]: The sixth node is a local minima because 1 is less than 5 and 2.
The minimum distance is between the fifth and the sixth node. minDistance = 6 - 5 = 1.
The maximum distance is between the third and the sixth node. maxDistance = 6 - 3 = 3.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/10/14/a5.png)
<pre>
<strong>Input:</strong> head = [1,3,2,2,3,2,2,2,7]
<strong>Output:</strong> [3,3]
<strong>Explanation:</strong> There are two critical points:
- [1,3,2,2,3,2,2,2,7]: The second node is a local maxima because 3 is greater than 1 and 2.
- [1,3,2,2,3,2,2,2,7]: The fifth node is a local maxima because 3 is greater than 2 and 2.
Both the minimum and maximum distances are between the second and the fifth node.
Thus, minDistance and maxDistance is 5 - 2 = 3.
Note that the last node is not considered a local maxima because it does not have a next node.
</pre>

#### Constraints:
* The number of nodes in the list is in the range <code>[2, 10<sup>5</sup>]</code>.
* <code>1 <= Node.val <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def nodesBetweenCriticalPoints(self, head: Optional[ListNode]) -> List[int]:
        prev = head
        curr = head.next
        firstlocal = -1
        prevlocal = -1
        i = 0
        ret = [-1, -1]

        while curr.next:
            if (prev.val < curr.val and curr.val > curr.next.val) \
                    or (prev.val > curr.val and curr.val < curr.next.val):
                if firstlocal == -1:
                    firstlocal = i
                elif ret[0] == -1:
                    ret = [i - prevlocal, i - firstlocal]
                else:
                    ret = [min(ret[0], i - prevlocal), i - firstlocal]
                prevlocal = i

            prev = curr
            curr = curr.next
            i += 1

        return ret
```
