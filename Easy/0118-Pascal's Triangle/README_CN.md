# 118. 杨辉三角
给定一个非负整数 *numRows*，生成杨辉三角的前 *numRows* 行。

![](https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif)<br>
在杨辉三角中，每个数是它左上方和右上方的数的和。

#### 示例:
<pre>
<strong>输入:</strong> 5
<strong>输出:</strong> 
[
     [1],
    [1,1],
   [1,2,1],
  [1,3,3,1],
 [1,4,6,4,1]
]
</pre>

## 题解 (Python)

### 1. 题解
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
