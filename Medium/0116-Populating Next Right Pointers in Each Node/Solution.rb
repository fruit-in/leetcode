# Definition for Node.
# class Node
#     attr_accessor :val, :left, :right, :next
#     def initialize(val)
#         @val = val
#         @left, @right, @next = nil, nil, nil
#     end
# end

# @param {Node} root
# @return {Node}
def connect(root)
    head = root

    while not head.nil?
        curr = head
        head = head.left

        while not curr.nil?
            if not curr.left.nil?
                curr.left.next = curr.right
                curr.right.next = curr.next.left if not curr.next.nil?
            end
            curr = curr.next
        end
    end

    return root
end
