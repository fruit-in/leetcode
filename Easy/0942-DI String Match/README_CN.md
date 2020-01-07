# 942. 增减字符串匹配
给定只含 ```"I"```（增大）或 ```"D"```（减小）的字符串 ```S``` ，令 ```N = S.length```。

返回 ```[0, 1, ..., N]``` 的任意排列 ```A``` 使得对于所有 ```i = 0, ..., N-1```，都有：
* 如果 ```S[i] == "I"```，那么 ```A[i] < A[i+1]```
* 如果 ```S[i] == "D"```，那么 ```A[i] > A[i+1]```

#### 示例 1:
<pre>
<strong>输入:</strong> "IDID"
<strong>输出:</strong> [0,4,1,3,2]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "III"
<strong>输出:</strong> [0,1,2,3]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "DDI"
<strong>输出:</strong> [3,2,0,1]
</pre>

#### 提示:
1. ```1 <= S.length <= 1000```
2. ```S``` 只包含字符 ```"I"``` 或 ```"D"```。

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def diStringMatch(self, S: str) -> List[int]:
        b, e = 0, len(S)
        A = []
        for di in S:
            if di == 'I':
                A.append(b)
                b += 1
            elif di == 'D':
                A.append(e)
                e -= 1
        A.append(b)
        return A
```
