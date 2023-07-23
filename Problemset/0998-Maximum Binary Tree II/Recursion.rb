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
# @param {Integer} val
# @return {TreeNode}
def insert_into_max_tree(root, val)
  if root.nil?
    TreeNode.new(val)
  elsif root.val < val
    TreeNode.new(val, root)
  else
    root.right = insert_into_max_tree(root.right, val)
    root
  end
end
