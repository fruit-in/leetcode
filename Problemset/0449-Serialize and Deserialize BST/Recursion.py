# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Codec:

    def serialize(self, root: TreeNode) -> str:
        if not root:
            return ""

        return chr(root.val) \
            + self.serialize(root.left) + self.serialize(root.right)

    def deserialize(self, data: str) -> TreeNode:
        if not data:
            return None

        i = next((i for i in range(len(data)) if data[i] > data[0]), len(data))

        root = TreeNode(ord(data[0]))
        root.left = self.deserialize(data[1:i])
        root.right = self.deserialize(data[i:])

        return root


# Your Codec object will be instantiated and called as such:
# Your Codec object will be instantiated and called as such:
# ser = Codec()
# deser = Codec()
# tree = ser.serialize(root)
# ans = deser.deserialize(tree)
# return ans
