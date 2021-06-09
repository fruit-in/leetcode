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
# @param {Integer[]} to_delete
# @return {TreeNode[]}
def del_nodes(root, to_delete)
  return [] if root.nil?

  ret = [root]
  left_ret = del_nodes(root.left, to_delete)
  right_ret = del_nodes(root.right, to_delete)

  return left_ret + right_ret if to_delete.include?(root.val)

  unless root.left.nil?
    if to_delete.include?(root.left.val)
      root.left = nil
      ret += left_ret
    else
      ret += left_ret[1..]
    end
  end
  unless root.right.nil?
    if to_delete.include?(root.right.val)
      root.right = nil
      ret += right_ret
    else
      ret += right_ret[1..]
    end
  end

  ret
end
