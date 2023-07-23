# 449. Serialize and Deserialize BST
Serialization is converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.

Design an algorithm to serialize and deserialize a **binary search tree**. There is no restriction on how your serialization/deserialization algorithm should work. You need to ensure that a binary search tree can be serialized to a string, and this string can be deserialized to the original tree structure.

**The encoded string should be as compact as possible.**

#### Example 1:
<pre>
<strong>Input:</strong> root = [2,1,3]
<strong>Output:</strong> [2,1,3]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> root = []
<strong>Output:</strong> []
</pre>

#### Constraints:
* The number of nodes in the tree is in the range <code>[0, 10<sup>4</sup>]</code>.
* <code>0 <= Node.val <= 10<sup>4</sup></code>
* The input tree is **guaranteed** to be a binary search tree.

## Solutions (Python)

### 1. Recursion
```Python
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
```
