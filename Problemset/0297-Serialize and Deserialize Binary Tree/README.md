# 297. Serialize and Deserialize Binary Tree
Serialization is the process of converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.

Design an algorithm to serialize and deserialize a binary tree. There is no restriction on how your serialization/deserialization algorithm should work. You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.

**Clarification:** The input/output format is the same as [how LeetCode serializes a binary tree](https://support.leetcode.com/hc/en-us/articles/360011883654-What-does-1-null-2-3-mean-in-binary-tree-representation-). You do not necessarily need to follow this format, so please be creative and come up with different approaches yourself.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/15/serdeser.jpg)
<pre>
<strong>Input:</strong> root = [1,2,3,null,null,4,5]
<strong>Output:</strong> [1,2,3,null,null,4,5]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> root = []
<strong>Output:</strong> []
</pre>

#### Constraints:
* The number of nodes in the tree is in the range <code>[0, 10<sup>4</sup>]</code>.
* `-1000 <= Node.val <= 1000`

## Solutions (Python)

### 1. Solution
```Python
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
```
