# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val)
#         @val = val
#         @left, @right = nil, nil
#     end
# end

# @param {TreeNode} root
# @param {TreeNode} p
# @param {TreeNode} q
# @return {TreeNode}
def lowest_common_ancestor(root, p, q)
    return nil if root.nil?
    return root if root.val == p.val or root.val == q.val

    ret_l = lowest_common_ancestor(root.left, p, q)
    ret_r = lowest_common_ancestor(root.right, p, q)

    if ret_l.nil? and ret_r.nil?
        return nil
    elsif ret_l.nil?
        return ret_r
    elsif ret_r.nil?
        return ret_l
    else
        return root
    end
end
