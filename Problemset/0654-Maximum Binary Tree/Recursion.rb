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
