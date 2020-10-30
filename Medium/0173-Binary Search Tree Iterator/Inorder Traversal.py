# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class BSTIterator:

    def __init__(self, root: TreeNode):
        self.stack = []
        self.curr = root

        while self.curr:
            self.stack.append(self.curr)
            self.curr = self.curr.left

    def next(self) -> int:
        """
        @return the next smallest number
        """
        self.curr = self.stack.pop()
        ret = self.curr.val

        self.curr = self.curr.right
        while self.curr:
            self.stack.append(self.curr)
            self.curr = self.curr.left

        return ret

    def hasNext(self) -> bool:
        """
        @return whether we have a next smallest number
        """
        return self.stack


# Your BSTIterator object will be instantiated and called as such:
# obj = BSTIterator(root)
# param_1 = obj.next()
# param_2 = obj.hasNext()
