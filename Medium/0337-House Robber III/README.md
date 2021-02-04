# 337. House Robber III
The thief has found himself a new place for his thievery again. There is only one entrance to this area, called the "root." Besides the root, each house has one and only one parent house. After a tour, the smart thief realized that "all houses in this place forms a binary tree". It will automatically contact the police if two directly-linked houses were broken into on the same night.

Determine the maximum amount of money the thief can rob tonight without alerting the police.

#### Example 1:
<pre>
<strong>Input:</strong> [3,2,3,null,3,null,1]

     <b>3</b>
    / \
   2   3
    \   \
     <b>3   1</b>
<strong>Output:</strong> 7
<strong>Explanation:</strong> Maximum amount of money the thief can rob = <b>3</b> + <b>3</b> + <b>1</b> = <b>7</b>.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [3,4,5,1,3,null,1]

     3
    / \
   <b>4   5</b>
  / \   \
 1   3   1
<strong>Output:</strong> 9
<strong>Explanation:</strong> Maximum amount of money the thief can rob = <b>4</b> + <b>5</b> = <b>9</b>.
</pre>

## Solutions (Python)

### 1. Recursion
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def rob(self, root: TreeNode) -> int:
        def foo(root: TreeNode) -> (int, int):
            if not root:
                return (0, 0)

            rob_left = foo(root.left)
            rob_right = foo(root.right)

            return (root.val + rob_left[1] + rob_right[1],
                    max(rob_left) + max(rob_right))

        return max(foo(root))
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
# @param {TreeNode} root
# @return {Integer}
def rob(root)
  foo(root).max
end

# @param {TreeNode} root
# @return {Integer[]}
def foo(root)
  return [0, 0] if root.nil?

  rob_left = foo(root.left)
  rob_right = foo(root.right)

  [root.val + rob_left[1] + rob_right[1], rob_left.max + rob_right.max]
end
```
