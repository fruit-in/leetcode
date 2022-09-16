# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class CBTInserter:

    def __init__(self, root: Optional[TreeNode]):
        self.root = root
        self.size = 0
        stack = [root]

        while stack:
            node = stack.pop()

            if node:
                self.size += 1
                stack.append(node.left)
                stack.append(node.right)

    def insert(self, val: int) -> int:
        self.size += 1
        size = self.size
        node = self.root
        stack = []

        while size > 1:
            stack.append(size & 1)
            size >>= 1

        while len(stack) > 1:
            node = node.left if stack.pop() == 0 else node.right

        if stack.pop() == 0:
            node.left = TreeNode(val)
        else:
            node.right = TreeNode(val)

        return node.val

    def get_root(self) -> Optional[TreeNode]:
        return self.root

# Your CBTInserter object will be instantiated and called as such:
# obj = CBTInserter(root)
# param_1 = obj.insert(val)
# param_2 = obj.get_root()
