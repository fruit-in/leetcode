# 119. Pascal's Triangle II
Given a non-negative index *k* where *k* ≤ 33, return the *k*<sup>th</sup> index row of the Pascal's triangle.

Note that the row index starts from 0.

![](https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif)<br>
In Pascal's triangle, each number is the sum of the two numbers directly above it.

#### Example 1:
<pre>
<strong>Input:</strong> 3
<strong>Output:</strong> [1,3,3,1]
</pre>

#### Follow up:
Could you optimize your algorithm to use only *O*(*k*) extra space?

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def getRow(self, rowIndex: int) -> List[int]:
        row = [1] * (rowIndex + 1)
        for i in range(2, rowIndex + 1):
            prenum = 1
            for j in range(1, i):
                row[j], prenum = row[j] + prenum, row[j]
            row[j + 1] = 1
        return row
```
