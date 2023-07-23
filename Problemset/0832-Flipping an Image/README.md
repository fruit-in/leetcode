# 832. Flipping an Image
Given a binary matrix <code>A</code>, we want to flip the image horizontally, then invert it, and return the resulting image.

To flip an image horizontally means that each row of the image is reversed.  For example, flipping <code>[1, 1, 0]</code> horizontally results in <code>[0, 1, 1]</code>.

To invert an image means that each <code>0</code> is replaced by <code>1</code>, and each <code>1</code> is replaced by <code>0</code>. For example, inverting <code>[0, 1, 1]</code> results in <code>[1, 0, 0]</code>.

#### Example 1:
<pre>
<strong>Input:</strong> [[1,1,0],[1,0,1],[0,0,0]]
<strong>Output:</strong> [[1,0,0],[0,1,0],[1,1,1]]
<strong>Explanation:</strong>
First reverse each row: [[0,1,1],[1,0,1],[0,0,0]].
Then, invert the image: [[1,0,0],[0,1,0],[1,1,1]]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [[1,1,0,0],[1,0,0,1],[0,1,1,1],[1,0,1,0]]
<strong>Output:</strong> [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
<strong>Explanation:</strong>
First reverse each row: [[0,0,1,1],[1,0,0,1],[1,1,1,0],[0,1,0,1]].
Then invert the image: [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
</pre>

#### Note:
* <code>1 <= A.length = A[0].length <= 20</code>
* <code>0 <= A[i][j] <= 1</code>

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def flipAndInvertImage(self, A: List[List[int]]) -> List[List[int]]:
        return [[0 if x == 1 else 1 for x in row] for row in map(reversed, A)]
```
