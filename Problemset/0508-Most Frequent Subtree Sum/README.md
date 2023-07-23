# 508. Most Frequent Subtree Sum
Given the root of a tree, you are asked to find the most frequent subtree sum. The subtree sum of a node is defined as the sum of all the node values formed by the subtree rooted at that node (including the node itself). So what is the most frequent subtree sum value? If there is a tie, return all the values with the highest frequency in any order.

#### Example 1:
Input:
```
  5
 /  \
2   -3
```
return [2, -3, 4], since all the values happen only once, return all of them in any order.

#### Example 2:
Input:
```
  5
 /  \
2   -5
```
return [2], since 2 happens twice, however -5 only occur once.

**Note:** You may assume the sum of values in any subtree is in the range of 32-bit signed integer. 

## Solutions (Python)

### 1. DFS
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findFrequentTreeSum(self, root: TreeNode) -> List[int]:
        def foo(root: TreeNode) -> int:
            sum = root.val
            sum += foo(root.left) if root.left else 0
            sum += foo(root.right) if root.right else 0

            freq[sum] = freq[sum] + 1 if sum in freq else 0

            return sum

        if not root:
            return []

        freq = {}
        foo(root)
        max_freq = max(freq.values())

        return [k for k, v in freq.items() if v == max_freq]
```
