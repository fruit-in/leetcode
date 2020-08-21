# 654. Maximum Binary Tree
Given an integer array with no duplicates. A maximum tree building on this array is defined as follow:
1. The root is the maximum number in the array.
2. The left subtree is the maximum tree constructed from left part subarray divided by the maximum number.
3. The right subtree is the maximum tree constructed from right part subarray divided by the maximum number.

Construct the maximum tree by the given array and output the root node of this tree.

#### Example:
<pre>
<strong>Input:</strong> [3,2,1,6,0,5]
<strong>Output:</strong> return the tree root node representing the following tree:

      6
    /   \
   3     5
    \    /
     2  0
       \
        1
</pre>

#### Note:
1. The size of the given array will be in the range [1,1000].

## Solutions (Python)

### 1. Recursion
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def constructMaximumBinaryTree(self, nums: List[int]) -> TreeNode:
        if nums:
            maxnum = max(nums)
            root = TreeNode(maxnum)
            index = nums.index(maxnum)
            root.left = self.constructMaximumBinaryTree(nums[:index])
            root.right = self.constructMaximumBinaryTree(nums[index + 1:])
            return root
        else:
            return None
```

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
# @param {Integer[]} nums
# @return {TreeNode}
def construct_maximum_binary_tree(nums)
    if not nums.empty?
        maxnum = nums.max
        root = TreeNode.new(maxnum)
        index = nums.index(maxnum)
        root.left = construct_maximum_binary_tree(nums[0...index])
        root.right = construct_maximum_binary_tree(nums[index + 1..-1])
        return root
    else
        return nil
    end
end
```
