# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class FindElements:

    def __init__(self, root: TreeNode):
        self.vals = set()
        root.val = 0
        stack = [root]

        while stack:
            curr = stack.pop()
            self.vals.add(curr.val)

            if curr.left:
                curr.left.val = 2 * curr.val + 1
                stack.append(curr.left)
            if curr.right:
                curr.right.val = 2 * curr.val + 2
                stack.append(curr.right)

    def find(self, target: int) -> bool:
        return target in self.vals


# Your FindElements object will be instantiated and called as such:
# obj = FindElements(root)
# param_1 = obj.find(target)
