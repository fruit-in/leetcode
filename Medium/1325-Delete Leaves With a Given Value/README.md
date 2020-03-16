# 1325. Delete Leaves With a Given Value
Given a binary tree ```root``` and an integer ```target```, delete all the **leaf nodes** with value ```target```.

Note that once you delete a leaf node with value ```target```, if it's parent node becomes a leaf node and has the value ```target```, it should also be deleted (you need to continue doing that until you can't).

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/01/09/sample_1_1684.png)
<pre>
<strong>Input:</strong> root = [1,2,3,2,null,2,4], target = 2
<strong>Output:</strong> [1,null,3,null,4]
<strong>Explanation:</strong> Leaf nodes in green with value (target = 2) are removed (Picture in left). 
After removing, new nodes become leaf nodes with value (target = 2) (Picture in center).
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/01/09/sample_2_1684.png)
<pre>
<strong>Input:</strong> root = [1,3,3,3,2], target = 3
<strong>Output:</strong> [1,3,null,null,2]
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2020/01/15/sample_3_1684.png)
<pre>
<strong>Input:</strong> root = [1,2,null,2,null,2], target = 2
<strong>Output:</strong> [1]
<strong>Explanation:</strong> Leaf nodes in green with value (target = 2) are removed at each step.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> root = [1,1,1], target = 1
<strong>Output:</strong> []
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> root = [1,2,3], target = 1
<strong>Output:</strong> [1,2,3]
</pre>

#### Constraints:
* ```1 <= target <= 1000```
* Each tree has at most ```3000``` nodes.
* Each node's value is between ```[1, 1000]```.

## Solutions (Python)

### 1. Postorder Traversal
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def removeLeafNodes(self, root: TreeNode, target: int) -> TreeNode:
        if not root:
            return None

        root.left = self.removeLeafNodes(root.left, target)
        root.right = self.removeLeafNodes(root.right, target)

        if not root.left and not root.right and root.val == target:
            return None

        return root
```
