# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def canMerge(self, trees: List[TreeNode]) -> Optional[TreeNode]:
        def buildTree(root: TreeNode) -> None:
            if root.left and root.left.val in trees:
                root.left = trees.pop(root.left.val)
                buildTree(root.left)
            if root.right and root.right.val in trees:
                root.right = trees.pop(root.right.val)
                buildTree(root.right)

        def isValid(root: TreeNode) -> (bool, int, int):
            minval, maxval = root.val, root.val
            if root.left:
                leftvalid, leftmin, leftmax = isValid(root.left)
                if not leftvalid or leftmax >= root.val:
                    return (False, minval, maxval)
                minval = leftmin
            if root.right:
                rightvalid, rightmin, rightmax = isValid(root.right)
                if not rightvalid or rightmin <= root.val:
                    return (False, minval, maxval)
                maxval = rightmax

            return (True, minval, maxval)

        trees = {tree.val: tree for tree in trees}
        leaves = set()

        for tree in trees.values():
            if tree.left:
                if tree.left.val in leaves:
                    return None
                leaves.add(tree.left.val)
            if tree.right:
                if tree.right.val in leaves:
                    return None
                leaves.add(tree.right.val)

        candidates = {tree for tree in trees if tree not in leaves}

        if len(candidates) != 1:
            return None

        root = trees.pop(candidates.pop())
        buildTree(root)

        if trees or not isValid(root)[0]:
            return None

        return root
