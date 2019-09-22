# 119. 杨辉三角 II
给定一个非负索引 *k*，其中 *k* ≤ 33，返回杨辉三角的第 *k* 行。

![](https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif)<br>
在杨辉三角中，每个数是它左上方和右上方的数的和。

#### 示例:
<pre>
<strong>输入:</strong> 3
<strong>输出:</strong> [1,3,3,1]
</pre>

#### 进阶:
你可以优化你的算法到 O(*k*) 空间复杂度吗？

## 题解 (Python)

### 1. 题解
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
