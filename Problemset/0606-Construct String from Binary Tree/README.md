# 606. Construct String from Binary Tree
You need to construct a string consists of parenthesis and integers from a binary tree with the preorder traversing way.

The null node needs to be represented by empty parenthesis pair "()". And you need to omit all the empty parenthesis pairs that don't affect the one-to-one mapping relationship between the string and the original binary tree.

#### Example 1:
<pre>
<strong>Input:</strong> Binary tree: [1,2,3,4]
       1
     /   \
    2     3
   /    
  4     
<strong>Output:</strong> "1(2(4))(3)"
<strong>Explanation:</strong> Originallay it needs to be "1(2(4)())(3()())", 
but you need to omit all the unnecessary empty parenthesis pairs. 
And it will be "1(2(4))(3)".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> Binary tree: [1,2,3,null,4]
       1
     /   \
    2     3
     \  
      4 
<strong>Output:</strong> "1(2()(4))(3)"
<strong>Explanation:</strong> Almost the same as the first example, 
except we can't omit the first parenthesis pair to break the one-to-one mapping relationship between the input and the output.
</pre>

## Solutions (Python)

### 1. Recursion
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def tree2str(self, t: TreeNode) -> str:
        if not t:
            return ""

        ret = str(t.val)

        if t.left or t.right:
            ret += "(" + self.tree2str(t.left) + ")"
        if t.right:
            ret += "(" + self.tree2str(t.right) + ")"

        return ret
```

### 2. Iteration
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def tree2str(self, t: TreeNode) -> str:
        if not t:
            return ""

        ret = ""
        nodes = [t]
        visited = set()

        while nodes:
            cur = nodes.pop()
            if cur not in visited:
                visited.add(cur)
                nodes.append(cur)
                ret += "(" + str(cur.val)

                if cur.right:
                    nodes.append(cur.right)
                if cur.left:
                    nodes.append(cur.left)
                if not cur.left and cur.right:
                    ret += "()"
            else:
                ret += ")"

        return ret[1:-1]
```
