# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def distanceK(self, root: TreeNode, target: TreeNode, k: int) -> List[int]:
        root.parent = None
        stack = [root]
        deque = collections.deque([(target, 0)])
        ret = []

        while stack != []:
            node = stack.pop()
            if node.left is not None:
                node.left.parent = node
                stack.append(node.left)
            if node.right is not None:
                node.right.parent = node
                stack.append(node.right)

        while len(deque) > 0:
            node, d = deque.popleft()

            if d > k:
                break
            elif d == k:
                ret.append(node.val)

            if node.left is not None:
                deque.append((node.left, d + 1))
                node.left.parent = None
            if node.right is not None:
                deque.append((node.right, d + 1))
                node.right.parent = None
            if node.parent is not None:
                deque.append((node.parent, d + 1))
                if node == node.parent.left:
                    node.parent.left = None
                else:
                    node.parent.right = None

        return ret
