# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
# @param {Integer} n
# @return {TreeNode[]}
def all_possible_fbt(n)
    return [] if n % 2 == 0
    return [TreeNode.new] if n == 1

    ret = []

    for i in (1..(n - 1)).step(2)
        for left in all_possible_fbt(i)
            for right in all_possible_fbt(n - 1 - i)
                ret.push(TreeNode.new(0, left, right))
            end
        end
    end

    return ret
end
