# 230. Kth Smallest Element in a BST
Given a binary search tree, write a function `kthSmallest` to find the **k**th smallest element in it.

#### Example 1:
<pre>
<b>Input:</b> root = [3,1,4,null,2], k = 1
   3
  / \
 1   4
  \
   2
<b>Output:</b> 1
</pre>

#### Example 2:
<pre>
<b>Input:</b> root = [5,3,6,2,4,null,null,1], k = 3
       5
      / \
     3   6
    / \
   2   4
  /
 1
<b>Output:</b> 3
</pre>

#### Follow up:
What if the BST is modified (insert/delete operations) often and you need to find the kth smallest frequently? How would you optimize the kthSmallest routine?

#### Constraints:
* The number of elements of the BST is between `1` to `10^4`.
* You may assume `k` is always valid, `1 ≤ k ≤ BST's total elements`.

## Solutions (Ruby)

### 1. Inorder Traversal
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
# @param {TreeNode} root
# @param {Integer} k
# @return {Integer}
def kth_smallest(root, k)
    stack = []

    while true
        until root.nil?
            stack.push(root)
            root = root.left
        end

        root = stack.pop
        k -= 1

        return root.val if k == 0

        root = root.right
    end
end
```
