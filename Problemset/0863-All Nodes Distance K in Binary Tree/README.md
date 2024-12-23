# 863. All Nodes Distance K in Binary Tree
Given the `root` of a binary tree, the value of a target node `target`, and an integer `k`, return *an array of the values of all nodes that have a distance* `k` *from the target node*.

You can return the answer in **any order**.

#### Example 1:
![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/06/28/sketch0.png)
<pre>
<strong>Input:</strong> root = [3,5,1,6,2,0,8,null,null,7,4], target = 5, k = 2
<strong>Output:</strong> [7,4,1]
<strong>Explanation:</strong> The nodes that are a distance 2 from the target node (with value 5) have values 7, 4, and 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> root = [1], target = 1, k = 3
<strong>Output:</strong> []
</pre>

#### Constraints:
* The number of nodes in the tree is in the range `[1, 500]`.
* `0 <= Node.val <= 500`
* All the values `Node.val` are **unique**.
* `target` is the value of one of the nodes in the tree.
* `0 <= k <= 1000`

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
    def distanceK(self, root: TreeNode, target: TreeNode, k: int) -> List[int]:
        root.parent = None
        stack = [root]
        deque = collections.deque([(target, 0)])
        ret = []

        while stack != []:
            node = stack.pop()
            if node.left is not None:
                node.left.parent = node
                stack.append(node.left)
            if node.right is not None:
                node.right.parent = node
                stack.append(node.right)

        while len(deque) > 0:
            node, d = deque.popleft()

            if d > k:
                break
            elif d == k:
                ret.append(node.val)

            if node.left is not None:
                deque.append((node.left, d + 1))
                node.left.parent = None
            if node.right is not None:
                deque.append((node.right, d + 1))
                node.right.parent = None
            if node.parent is not None:
                deque.append((node.parent, d + 1))
                if node == node.parent.left:
                    node.parent.left = None
                else:
                    node.parent.right = None

        return ret
```
