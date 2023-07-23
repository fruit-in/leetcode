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
# @param {Integer} key
# @return {TreeNode}
def delete_node(root, key)
  return nil if root.nil?

  if root.val == key
    root = merge(root.left, root.right)
  elsif root.val > key
    root.left = delete_node(root.left, key)
  else
    root.right = delete_node(root.right, key)
  end

  root
end

# @param {TreeNode} left
# @param {TreeNode} right
# @return {TreeNode}
def merge(left, right)
  return right if left.nil?

  curr = left
  curr = curr.right until curr.right.nil?
  curr.right = right

  left
end
