# 2196. Create Binary Tree From Descriptions
You are given a 2D integer array `descriptions` where <code>descriptions[i] = [parent<sup>i</sup>, child<sup>i</sup>, isLeft<sup>i</sup>]</code> indicates that <code>parent<sup>i</sup></code> is the **parent** of <code>child<sup>i</sup></code> in a **binary** tree of **unique** values. Furthermore,

* If <code>isLeft<sup>i</sup> == 1</code>, then <code>child<sup>i</sup></code> is the left child of <code>parent<sup>i</sup></code>.
* If <code>isLeft<sup>i</sup> == 0</code>, then <code>child<sup>i</sup></code> is the right child of <code>parent<sup>i</sup></code>.

Construct the binary tree described by `descriptions` and return *its **root***.

The test cases will be generated such that the binary tree is **valid**.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/02/09/example1drawio.png)
<pre>
<strong>Input:</strong> descriptions = [[20,15,1],[20,17,0],[50,20,1],[50,80,0],[80,19,1]]
<strong>Output:</strong> [50,20,80,15,17,19]
<strong>Explanation:</strong> The root node is the node with value 50 since it has no parent.
The resulting binary tree is shown in the diagram.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/02/09/example2drawio.png)
<pre>
<strong>Input:</strong> descriptions = [[1,2,1],[2,3,0],[3,4,1]]
<strong>Output:</strong> [1,2,null,null,3,4]
<strong>Explanation:</strong> The root node is the node with value 1 since it has no parent.
The resulting binary tree is shown in the diagram.
</pre>

#### Constraints:
* <code>1 <= descriptions.length <= 10<sup>4</sup></code>
* `descriptions[i].length == 3`
* <code>1 <= parent<sup>i</sup>, child<sup>i</sup> <= 10<sup>5</sup></code>
* <code>0 <= isLeft<sup>i</sup> <= 1</code>
* The binary tree described by `descriptions` is valid.

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
    def createBinaryTree(self, descriptions: List[List[int]]) -> Optional[TreeNode]:
        parents = set()
        children = set()
        nodes = {}

        for parent, child, isleft in descriptions:
            parents.add(parent)
            children.add(child)

            if parent not in nodes:
                nodes[parent] = TreeNode(parent)
            if child not in nodes:
                nodes[child] = TreeNode(child)

            if isleft:
                nodes[parent].left = nodes[child]
            else:
                nodes[parent].right = nodes[child]

        return nodes[(parents - children).pop()]
```
