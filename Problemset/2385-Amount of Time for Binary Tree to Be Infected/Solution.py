# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def amountOfTime(self, root: Optional[TreeNode], start: int) -> int:
        def setParent(root: Optional[TreeNode]) -> None:
            if root is None:
                return

            if root.left is not None:
                root.left.parent = root
                setParent(root.left)
            if root.right is not None:
                root.right.parent = root
                setParent(root.right)

        def findNode(root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
            if root is None:
                return
            if root.val == val:
                return root
            findleft = findNode(root.left, val)
            if findleft is not None:
                return findleft
            return findNode(root.right, val)

        setParent(root)
        root.parent = None
        infected = {start}
        deque = collections.deque([(findNode(root, start), 0)])
        ret = 0

        while len(deque) > 0:
            node, minutes = deque.popleft()
            ret = minutes
            if node.parent is not None and node.parent.val not in infected:
                infected.add(node.parent.val)
                deque.append((node.parent, minutes + 1))
            if node.left is not None and node.left.val not in infected:
                infected.add(node.left.val)
                deque.append((node.left, minutes + 1))
            if node.right is not None and node.right.val not in infected:
                infected.add(node.right.val)
                deque.append((node.right, minutes + 1))

        return ret
