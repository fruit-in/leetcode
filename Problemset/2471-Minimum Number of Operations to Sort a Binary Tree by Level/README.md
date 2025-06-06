# 2471. Minimum Number of Operations to Sort a Binary Tree by Level
You are given the `root` of a binary tree with **unique values**.

In one operation, you can choose any two nodes **at the same level** and swap their values.

Return *the minimum number of operations needed to make the values at each level sorted in a **strictly increasing order***.

The **level** of a node is the number of edges along the path between it and the root node.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/09/18/image-20220918174006-2.png)
<pre>
<strong>Input:</strong> root = [1,4,3,7,6,8,5,null,null,null,null,9,null,10]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
- Swap 4 and 3. The 2nd level becomes [3,4].
- Swap 7 and 5. The 3rd level becomes [5,6,8,7].
- Swap 8 and 7. The 3rd level becomes [5,6,7,8].
We used 3 operations so return 3.
It can be proven that 3 is the minimum number of operations needed.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/09/18/image-20220918174026-3.png)
<pre>
<strong>Input:</strong> root = [1,3,2,7,6,5,4]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
- Swap 3 and 2. The 2nd level becomes [2,3].
- Swap 7 and 4. The 3rd level becomes [4,6,5,7].
- Swap 6 and 5. The 3rd level becomes [4,5,6,7].
We used 3 operations so return 3.
It can be proven that 3 is the minimum number of operations needed.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2022/09/18/image-20220918174052-4.png)
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,6]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Each level is already sorted in increasing order so return 0.
</pre>

#### Constraints:
* The number of nodes in the tree is in the range <code>[1, 10<sup>5</sup>]</code>.
* <code>1 <= Node.val <= 10<sup>5</sup></code>
* All the values of the tree are **unique**.

## Solutions (Python)

### 1. Solution
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def minimumOperations(self, root: Optional[TreeNode]) -> int:
        nodes = [root]
        ret = 0

        while nodes:
            nextlevel = []
            vals = []
            heap = []

            for node in nodes:
                if node.left:
                    nextlevel.append(node.left)
                    vals.append(node.left.val)
                    heapq.heappush(heap, (vals[-1], len(vals) - 1))
                if node.right:
                    nextlevel.append(node.right)
                    vals.append(node.right.val)
                    heapq.heappush(heap, (vals[-1], len(vals) - 1))

            for i in range(len(vals)):
                while heap[0][0] != vals[heap[0][1]]:
                    heapq.heappop(heap)

                j = heapq.heappop(heap)[1]
                if i != j:
                    heapq.heappush(heap, (vals[i], j))
                    vals[i], vals[j] = vals[j], vals[i]
                    ret += 1

            nodes = nextlevel

        return ret
```
