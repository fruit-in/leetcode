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
