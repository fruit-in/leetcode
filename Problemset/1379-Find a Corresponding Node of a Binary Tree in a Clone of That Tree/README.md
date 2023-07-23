# 1379. Find a Corresponding Node of a Binary Tree in a Clone of That Tree
Given two binary trees ```original``` and ```cloned``` and given a reference to a node ```target``` in the original tree.

The ```cloned``` tree is a **copy of** the ```original``` tree.

Return *a reference to the same node* in the ```cloned``` tree.

**Note** that you are **not allowed** to change any of the two trees or the ```target``` node and the answer **must be** a reference to a node in the ```cloned``` tree.

**Follow up:** Solve the problem if repeated values on the tree are allowed.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/02/21/e1.png)
<pre>
<strong>Input:</strong> tree = [7,4,3,null,null,6,19], target = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> In all examples the original and cloned trees are shown. The target node is a green node from the original tree. The answer is the yellow node from the cloned tree.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/02/21/e2.png)
<pre>
<strong>Input:</strong> tree = [7], target =  7
<strong>Output:</strong> 7
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2020/02/21/e3.png)
<pre>
<strong>Input:</strong> tree = [8,null,6,null,5,null,4,null,3,null,2,null,1], target = 4
<strong>Output:</strong> 4
</pre>

#### Example 4:
![](https://assets.leetcode.com/uploads/2020/02/21/e4.png)
<pre>
<strong>Input:</strong> tree = [1,2,3,4,5,6,7,8,9,10], target = 5
<strong>Output:</strong> 5
</pre>

#### Example 5:
![](https://assets.leetcode.com/uploads/2020/02/21/e5.png)
<pre>
<strong>Input:</strong> tree = [1,2,null,3], target = 2
<strong>Output:</strong> 2
</pre>

#### Constraints:
* The number of nodes in the ```tree``` is in the range ```[1, 10^4]```.
* The values of the nodes of the ```tree``` are unique.
* ```target``` node is a node from the ```original``` tree and is not ```null```.

## Solutions (Python)

### 1. Solution
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def getTargetCopy(self, original: TreeNode, cloned: TreeNode, target: TreeNode) -> TreeNode:
        nodes = [original, cloned]

        while nodes:
            clo_curr = nodes.pop()
            ori_curr = nodes.pop()

            if ori_curr == target:
                return clo_curr

            if ori_curr.left:
                nodes.append(ori_curr.left)
                nodes.append(clo_curr.left)
            if ori_curr.right:
                nodes.append(ori_curr.right)
                nodes.append(clo_curr.right)
```
