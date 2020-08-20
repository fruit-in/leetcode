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
# @return {TreeNode}
def prune_tree(root)
    return nil if root.nil?

    root.left = prune_tree(root.left)
    root.right = prune_tree(root.right)

    if root.val == 1 or not root.left.nil? or not root.right.nil?
        return root
    else
        return nil
    end
end
