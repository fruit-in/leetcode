# 118. Pascal's Triangle
Given a non-negative integer *numRows*, generate the first *numRows* of Pascal's triangle.

![](https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif)<br>
In Pascal's triangle, each number is the sum of the two numbers directly above it.

#### Example 1:
<pre>
<strong>Input:</strong> 5
<strong>Output:</strong> 
[
     [1],
    [1,1],
   [1,2,1],
  [1,3,3,1],
 [1,4,6,4,1]
]
</pre>

## Solutions

### 1. Solution (Python3)
```Python3
class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        tri = []
        for i in range(numRows):
            row = [1]
            for j in range(i - 1):
                row.append(tri[-1][j] + tri[-1][j + 1])
            if i:
                row.append(1)
            tri.append(row)
        return tri
```
