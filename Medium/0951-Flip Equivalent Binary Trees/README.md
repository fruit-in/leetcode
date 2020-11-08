# 951. Flip Equivalent Binary Trees
For a binary tree **T**, we can define a **flip operation** as follows: choose any node, and swap the left and right child subtrees.

A binary tree **X** is *flip equivalent* to a binary tree **Y** if and only if we can make **X** equal to **Y** after some number of flip operations.

Given the roots of two binary trees `root1` and `root2`, return `true` if the two trees are flip equivelent or `false` otherwise.

#### Example 1:
![](https://assets.leetcode.com/uploads/2018/11/29/tree_ex.png)
<pre>
<b>Input:</b> root1 = [1,2,3,4,5,6,null,null,null,7,8], root2 = [1,3,2,null,6,4,5,null,null,null,null,8,7]
<b>Output:</b> true
<b>Explanation:</b> We flipped at nodes with values 1, 3, and 5.
</pre>

#### Example 2:
<pre>
<b>Input:</b> root1 = [], root2 = []
<b>Output:</b> true
</pre>

#### Example 3:
<pre>
<b>Input:</b> root1 = [], root2 = [1]
<b>Output:</b> false
</pre>

#### Example 4:
<pre>
<b>Input:</b> root1 = [0,null,1], root2 = []
<b>Output:</b> false
</pre>

#### Example 5:
<pre>
<b>Input:</b> root1 = [0,null,1], root2 = [0,1]
<b>Output:</b> true
</pre>

#### Constraints:
* The number of nodes in each tree is in the range `[0, 100]`.
* Each tree will have **unique node values** in the range `[0, 99]`.

## Solutions (Ruby)

### 1. Recursion
```Ruby
# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
# @param {TreeNode} root1
# @param {TreeNode} root2
# @return {Boolean}
def flip_equiv(root1, root2)
    return true if root1.nil? and root2.nil?
    return false if root1.nil? or root2.nil? or root1.val != root2.val
    return true if flip_equiv(root1.left, root2.left) and flip_equiv(root1.right, root2.right)
    return true if flip_equiv(root1.left, root2.right) and flip_equiv(root1.right, root2.left)
    return false
end
```
