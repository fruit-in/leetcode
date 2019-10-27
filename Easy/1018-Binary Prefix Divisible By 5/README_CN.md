# 1018. 可被 5 整除的二进制前缀
给定由若干 ```0``` 和 ```1``` 组成的数组 ```A```。我们定义 ```N_i```：从 ```A[0]``` 到 ```A[i]``` 的第 ```i``` 个子数组被解释为一个二进制数（从最高有效位到最低有效位）。

返回布尔值列表 ```answer```，只有当 ```N_i``` 可以被 ```5``` 整除时，答案 ```answer[i]``` 为 ```true```，否则为 ```false```。

#### 示例 1:
<pre>
<strong>输入:</strong> [0,1,1]
<strong>输出:</strong> [true,false,false]
<strong>解释:</strong>
输入数字为 0, 01, 011；也就是十进制中的 0, 1, 3 。只有第一个数可以被 5 整除，因此 answer[0] 为真。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,1,1]
<strong>输出:</strong> [false,false,false]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [0,1,1,1,1,1]
<strong>输出:</strong> [true,false,false,false,true,false]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> [1,1,1,0,1]
<strong>输出:</strong> [false,false,false,false,false]
</pre>

#### 提示:
1. <code>1 <= A.length <= 30000</code>
2. <code>A[i]</code> 为 <code>0</code> 或 <code>1</code>

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def prefixesDivBy5(self, A: List[int]) -> List[bool]:
        n = 0
        for i in range(len(A)):
            n = (2 * n + A[i]) % 5
            A[i] = n == 0
        return A
```
