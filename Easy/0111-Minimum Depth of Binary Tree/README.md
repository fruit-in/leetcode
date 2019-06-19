# 111. Minimum Depth of Binary Tree
Given a binary tree, find its minimum depth.

The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.

**Note:** A leaf is a node with no children.

#### Example:
<pre>
Given binary tree [3,9,20,null,null,15,7],

    3
   / \
  9  20
    /  \
   15   7

return its minimum depth = 2.
</pre>

## Solutions

### 1. Recursion (C)
```C
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

int minDepth(struct TreeNode* root){
    int lmin = 0, rmin = 0;
    if(root == NULL)
        return 0;
    if(root -> left == NULL && root -> right == NULL)
        return 1;
    if(root -> left != NULL)
        lmin = minDepth(root -> left) + 1;
    if(root -> right != NULL)
        rmin = minDepth(root -> right) + 1;
    if(lmin != 0 && rmin != 0)
        return lmin > rmin ? rmin : lmin;
    else
        return lmin == 0 ? rmin : lmin;
}
```

### 2. Iteration (Python3)
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def minDepth(self, root: TreeNode) -> int:
        if not root:
            return 0
        depth = 1
        nodes = [root]
        while True:
            for node in nodes:
                if not node.left and not node.right:
                    return depth
            depth += 1
            nodes = [node.left for node in nodes if node.left] \
                + [node.right for node in nodes if node.right]
```
