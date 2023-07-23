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
