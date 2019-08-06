# 867. Transpose Matrix
Given a matrix <code>A</code>, return the transpose of <code>A</code>.

The transpose of a matrix is the matrix flipped over it's main diagonal, switching the row and column indices of the matrix.

#### Example 1:
<pre>
<strong>Input:</strong> [[1,2,3],[4,5,6],[7,8,9]]
<strong>Output:</strong> [[1,4,7],[2,5,8],[3,6,9]]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [[1,2,3],[4,5,6]]
<strong>Output:</strong> [[1,4],[2,5],[3,6]]
</pre>

#### Note:
1. <code>1 <= A.length <= 1000</code>
2. <code>1 <= A[0].length <= 1000</code>

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def transpose(self, A: List[List[int]]) -> List[List[int]]:
        return list(zip(*A))
```
