# 558. Quad Tree Intersection
A quadtree is a tree data in which each internal node has exactly four children: ```topLeft```, ```topRight```, ```bottomLeft``` and ```bottomRight```. Quad trees are often used to partition a two-dimensional space by recursively subdividing it into four quadrants or regions.

We want to store True/False information in our quad tree. The quad tree is used to represent a ```N * N``` boolean grid. For each node, it will be subdivided into four children nodes **until the values in the region it represents are all the same**. Each node has another two boolean attributes : ```isLeaf``` and ```val```. ```isLeaf``` is true if and only if the node is a leaf node. The ```val``` attribute for a leaf node contains the value of the region it represents.

For example, below are two quad trees A and B:
```
A:
+-------+-------+   T: true
|       |       |   F: false
|   T   |   T   |
|       |       |
+-------+-------+
|       |       |
|   F   |   F   |
|       |       |
+-------+-------+
topLeft: T
topRight: T
bottomLeft: F
bottomRight: F

B:
+-------+---+---+
|       | F | F |
|   T   +---+---+
|       | T | T |
+-------+---+---+
|       |       |
|   T   |   F   |
|       |       |
+-------+-------+
topLeft: T
topRight:
     topLeft: F
     topRight: F
     bottomLeft: T
     bottomRight: T
bottomLeft: T
bottomRight: F
```

Your task is to implement a function that will take two quadtrees and return a quadtree that represents the logical OR (or union) of the two trees.
```
A:                 B:                 C (A or B):
+-------+-------+  +-------+---+---+  +-------+-------+
|       |       |  |       | F | F |  |       |       |
|   T   |   T   |  |   T   +---+---+  |   T   |   T   |
|       |       |  |       | T | T |  |       |       |
+-------+-------+  +-------+---+---+  +-------+-------+
|       |       |  |       |       |  |       |       |
|   F   |   F   |  |   T   |   F   |  |   T   |   F   |
|       |       |  |       |       |  |       |       |
+-------+-------+  +-------+-------+  +-------+-------+
```

#### Note:
1. Both ```A``` and ```B``` represent grids of size ```N * N```.
2. ```N``` is guaranteed to be a power of 2.
3. If you want to know more about the quad tree, you can refer to its [wiki](https://en.wikipedia.org/wiki/Quadtree).
4. The logic OR operation is defined as this: "A or B" is true if ```A is true```, or if ```B is true```, or if ```both A and B are true```.

## Solutions (Python)

### 1. Recursion
```Python3
"""
# Definition for a QuadTree node.
class Node:
    def __init__(self, val, isLeaf, topLeft, topRight, bottomLeft, bottomRight):
        self.val = val
        self.isLeaf = isLeaf
        self.topLeft = topLeft
        self.topRight = topRight
        self.bottomLeft = bottomLeft
        self.bottomRight = bottomRight
"""
class Solution:
    def intersect(self, quadTree1: 'Node', quadTree2: 'Node') -> 'Node':
        if quadTree1.isLeaf and quadTree2.isLeaf:
            quadTree1.val |= quadTree2.val
            return quadTree1
        elif quadTree1.isLeaf:
            return quadTree1 if quadTree1.val else quadTree2
        elif quadTree2.isLeaf:
            return quadTree2 if quadTree2.val else quadTree1
        else:
            quadTree1.topLeft = self.intersect(quadTree1.topLeft, quadTree2.topLeft)
            quadTree1.topRight = self.intersect(quadTree1.topRight, quadTree2.topRight)
            quadTree1.bottomLeft = self.intersect(quadTree1.bottomLeft, quadTree2.bottomLeft)
            quadTree1.bottomRight = self.intersect(quadTree1.bottomRight, quadTree2.bottomRight)

            if (quadTree1.topLeft.isLeaf
                and quadTree1.topRight.isLeaf
                and quadTree1.bottomLeft.isLeaf
                and quadTree1.bottomRight.isLeaf
                and quadTree1.topLeft.val
                == quadTree1.topRight.val
                == quadTree1.bottomLeft.val
                == quadTree1.bottomRight.val):
                quadTree1 = quadTree1.topLeft

            return quadTree1
```
