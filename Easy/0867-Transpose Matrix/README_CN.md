# 867. 转置矩阵
给定一个矩阵 ```A```， 返回 ```A``` 的转置矩阵。

矩阵的转置是指将矩阵的主对角线翻转，交换矩阵的行索引与列索引。

#### 示例 1:
<pre>
<strong>输入:</strong> [[1,2,3],[4,5,6],[7,8,9]]
<strong>输出:</strong> [[1,4,7],[2,5,8],[3,6,9]]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [[1,2,3],[4,5,6]]
<strong>输出:</strong> [[1,4],[2,5],[3,6]]
</pre>

#### 提示:
1. ```1 <= A.length <= 1000```
2. ```1 <= A[0].length <= 1000```

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def transpose(self, A: List[List[int]]) -> List[List[int]]:
        return list(zip(*A))
```
