# 655. Print Binary Tree
Print a binary tree in an m*n 2D string array following these rules:
1. The row number `m` should be equal to the height of the given binary tree.
2. The column number `n` should always be an odd number.
3. The root node's value (in string format) should be put in the exactly middle of the first row it can be put. The column and the row where the root node belongs will separate the rest space into two parts (**left-bottom part and right-bottom part**). You should print the left subtree in the left-bottom part and print the right subtree in the right-bottom part. The left-bottom part and the right-bottom part should have the same size. Even if one subtree is none while the other is not, you don't need to print anything for the none subtree but still need to leave the space as large as that for the other subtree. However, if two subtrees are none, then you don't need to leave space for both of them. 
4. Each unused space should contain an empty string `""`.
5. Print the subtrees following the same rules.

#### Example 1:
<pre>
<b>Input:</b>
     1
    /
   2
<b>Output:</b>
[["", "1", ""],
 ["2", "", ""]]
</pre>

#### Example 2:
<pre>
<b>Input:</b>
     1
    / \
   2   3
    \
     4
<b>Output:</b>
[["", "", "", "1", "", "", ""],
 ["", "2", "", "", "", "3", ""],
 ["", "", "4", "", "", "", ""]]
</pre>

#### Example 3:
<pre>
<b>Input:</b>
      1
     / \
    2   5
   /
  3
 /
4
<b>Output:</b>
[["",  "",  "", "",  "", "", "", "1", "",  "",  "",  "",  "", "", ""]
 ["",  "",  "", "2", "", "", "", "",  "",  "",  "",  "5", "", "", ""]
 ["",  "3", "", "",  "", "", "", "",  "",  "",  "",  "",  "", "", ""]
 ["4", "",  "", "",  "", "", "", "",  "",  "",  "",  "",  "", "", ""]]
</pre>

#### Note:
The height of binary tree is in the range of [1, 10].

## Solutions (Python)

### 1. Level Order Traversal
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def printTree(self, root: TreeNode) -> List[List[str]]:
        curr_level = [root]
        vals = []

        while any(curr_level):
            curr_vals = []
            next_level = []

            for node in curr_level:
                if node:
                    curr_vals.append(str(node.val))
                    next_level.append(node.left)
                    next_level.append(node.right)
                else:
                    curr_vals.append("")
                    next_level.append(None)
                    next_level.append(None)

            vals.append(curr_vals)
            curr_level = next_level

        indices = [i for i in range(0, 2 ** len(vals), 2)]
        ret = [[""] * (2 ** len(vals) - 1) for _ in range(len(vals))]

        for i in range(-1, -1 - len(vals), -1):
            new_indices = []

            while indices:
                index0 = indices.pop()
                ret[i][index0] = vals[i].pop()
                if indices:
                    index1 = indices.pop()
                    ret[i][index1] = vals[i].pop()
                    new_indices.append((index0 + index1) // 2)

            indices = new_indices[::-1]

        return ret
```
