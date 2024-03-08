# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Codec:

    def serialize(self, root):
        """Encodes a tree to a single string.

        :type root: TreeNode
        :rtype: str
        """
        nodes = [root]
        i = 0

        while i < len(nodes):
            if nodes[i] is not None:
                nodes.append(nodes[i].left)
                nodes.append(nodes[i].right)

            i += 1

        return ','.join('N' if node is None else str(node.val) for node in nodes)

    def deserialize(self, data):
        """Decodes your encoded data to tree.

        :type data: str
        :rtype: TreeNode
        """
        nodes = [TreeNode(int(x)) if x !=
                 'N' else None for x in data.split(',')]
        i = 0
        j = 1

        while j < len(nodes):
            nodes[i].left = nodes[j]
            nodes[i].right = nodes[j + 1]
            i += 1
            j += 2
            while i < j and nodes[i] is None:
                i += 1

        return nodes[0]

# Your Codec object will be instantiated and called as such:
# ser = Codec()
# deser = Codec()
# ans = deser.deserialize(ser.serialize(root))
