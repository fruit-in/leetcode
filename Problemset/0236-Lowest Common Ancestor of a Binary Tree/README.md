# 236. Lowest Common Ancestor of a Binary Tree
Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.

According to the [definition of LCA on Wikipedia](https://en.wikipedia.org/wiki/Lowest_common_ancestor): “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow **a node to be a descendant of itself**).”

#### Example 1:
![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)
<pre>
<b>Input:</b> root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
<b>Output:</b> 3
<b>Explanation:</b> The LCA of nodes 5 and 1 is 3.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)
<pre>
<b>Input:</b> root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
<b>Output:</b> 5
<b>Explanation:</b> The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.
</pre>

#### Example 3:
<pre>
<b>Input:</b> root = [1,2], p = 1, q = 2
<b>Output:</b> 1
</pre>

#### Constraints:
* The number of nodes in the tree is in the range <code>[2, 10<sup>5</sup>]</code>.
* <code>-10<sup>9</sup> <= Node.val <= 10<sup>9</sup></code>
* All `Node.val` are **unique**.
* `p != q`
* `p` and `q` will exist in the tree.

## Solutions (Ruby)

### 1. Recursion
```Ruby
# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val)
#         @val = val
#         @left, @right = nil, nil
#     end
# end

# @param {TreeNode} root
# @param {TreeNode} p
# @param {TreeNode} q
# @return {TreeNode}
def lowest_common_ancestor(root, p, q)
    return nil if root.nil?
    return root if root.val == p.val or root.val == q.val

    ret_l = lowest_common_ancestor(root.left, p, q)
    ret_r = lowest_common_ancestor(root.right, p, q)

    if ret_l.nil? and ret_r.nil?
        return nil
    elsif ret_l.nil?
        return ret_r
    elsif ret_r.nil?
        return ret_l
    else
        return root
    end
end
```
