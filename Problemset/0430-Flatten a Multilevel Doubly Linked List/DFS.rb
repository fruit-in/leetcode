# Definition for a Node.
# class Node
#     attr_accessor :val, :prev, :next, :child
#     def initialize(val=nil, prev=nil, next_=nil, child=nil)
#         @val = val
#         @prev = prev
#         @next = next_
#         @child = child
#     end
# end

# @param {Node} root
# @return {Node}
def flatten(root)
    return nil if root.nil?

    prev = Node.new
    stack = [root]

    until stack.empty?
        curr = stack.pop

        stack.push(curr.next) unless curr.next.nil?
        stack.push(curr.child) unless curr.child.nil?

        prev.next = curr
        curr.prev = prev
        curr.child = nil

        prev = curr
    end

    root.prev = nil

    return root
end
