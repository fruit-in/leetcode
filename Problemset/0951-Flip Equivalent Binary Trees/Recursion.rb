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
