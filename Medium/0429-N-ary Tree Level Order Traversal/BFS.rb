# Definition for a Node.
# class Node
#     attr_accessor :val, :children
#     def initialize(val)
#         @val = val
#         @children = []
#     end
# end

# @param {Node} root
# @return {List[List[int]]}
def levelOrder(root)
    return [] if root.nil?

    ret = []
    nodes = [root]

    while not nodes.empty?
        level = []
        temp = []
        for curr in nodes
            level.push(curr.val)
            temp.concat(curr.children)
        end
        nodes = temp
        ret.push(level)
    end

    return ret
end
