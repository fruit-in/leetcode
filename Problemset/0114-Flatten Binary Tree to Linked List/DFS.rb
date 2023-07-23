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
# @return {Void} Do not return anything, modify root in-place instead.
def flatten(root)
    return nil if root.nil?

    stack = [root]
    prev = TreeNode.new(left=root)

    while not stack.empty?
        curr = stack.pop

        prev.left = nil
        prev.right = curr

        stack.push(curr.right) if not curr.right.nil?
        stack.push(curr.left) if not curr.left.nil?

        prev = curr
    end
end
