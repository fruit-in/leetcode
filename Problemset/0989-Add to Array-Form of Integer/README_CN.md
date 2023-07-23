# 989. 数组形式的整数加法
对于非负整数 ```X``` 而言，*```X``` 的数组形式*是每位数字按从左到右的顺序形成的数组。例如，如果 ```X = 1231```，那么其数组形式为 ```[1,2,3,1]```。

给定非负整数 ```X``` 的数组形式 ```A```，返回整数 ```X+K``` 的数组形式。

#### 示例 1:
<pre>
<strong>输入:</strong> A = [1,2,0,0], K = 34
<strong>输出:</strong> [1,2,3,4]
<strong>解释:</strong> 1200 + 34 = 1234
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> A = [2,7,4], K = 181
<strong>输出:</strong> [4,5,5]
<strong>解释:</strong> 274 + 181 = 455
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> A = [2,1,5], K = 806
<strong>输出:</strong> [1,0,2,1]
<strong>解释:</strong> 215 + 806 = 1021
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> A = [9,9,9,9,9,9,9,9,9,9], K = 1
<strong>输出:</strong> [1,0,0,0,0,0,0,0,0,0,0]
<strong>解释:</strong> 9999999999 + 1 = 10000000000
</pre>

#### 提示:
1. ```1 <= A.length <= 10000```
2. ```0 <= A[i] <= 9```
3. ```0 <= K <= 10000```
4. 如果 ```A.length > 1```，那么 ```A[0] != 0```

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def addToArrayForm(self, A: List[int], K: int) -> List[int]:
        A[-1] += K
        i = -1
        while A[i] > 9:
            if len(A) > -i:
                A[i - 1] += A[i] // 10
            else:
                A = [A[i] // 10] + A
            A[i] %= 10
            i -= 1
        return A
```
