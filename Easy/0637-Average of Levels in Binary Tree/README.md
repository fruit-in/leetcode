# 637. Average of Levels in Binary Tree
Given a non-empty binary tree, return the average value of the nodes on each level in the form of an array.

#### Example 1:
<pre>
<strong>Input:</strong>
    3
   / \
  9  20
    /  \
   15   7
<strong>Output:</strong> [3, 14.5, 11]
<strong>Explanation:</strong>
The average value of nodes on level 0 is 3,  on level 1 is 14.5, and on level 2 is 11. Hence return [3, 14.5, 11].
</pre>

#### Note:
1. The range of node's value is in the range of 32-bit signed integer.

## Solutions (Python)

### 1. BFS
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def averageOfLevels(self, root: TreeNode) -> List[float]:
        averages = []
        curr_level = [root]

        while curr_level:
            averages.append(sum(node.val for node in curr_level) / len(curr_level))

            temp = [node.left for node in curr_level if node.left]
            temp.extend(node.right for node in curr_level if node.right)
            curr_level = temp

        return averages
```
