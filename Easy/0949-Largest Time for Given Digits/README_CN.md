# 949. 给定数字能组成的最大时间
给定一个由 4 位数字组成的数组，返回可以设置的符合 24 小时制的最大时间。

最小的 24 小时制时间是 00:00，而最大的是 23:59。从 00:00 （午夜）开始算起，过得越久，时间越大。

以长度为 5 的字符串返回答案。如果不能确定有效时间，则返回空字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,3,4]
<strong>输出:</strong> "23:41"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [5,5,5,5]
<strong>输出:</strong> ""
</pre>

#### 提示:
1. ```A.length == 4```
2. ```0 <= A[i] <= 9```

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def largestTimeFromDigits(self, A: List[int]) -> str:
        cnt0_3 = sum(1 if x < 4 else 0 for x in A)
        cnt0_5 = sum(1 if x < 6 else 0 for x in A)
        if 2 in A and cnt0_3 > 1 and cnt0_5 > 2:
            A.remove(2)
            hour = "2" + str(max(filter(lambda x: x < 4, A)))
            A.remove(max(filter(lambda x: x < 4, A)))
        elif 1 in A:
            A.remove(1)
            hour = "1" + str(max(A))
            A.remove(max(A))
        elif 0 in A:
            A.remove(0)
            hour = "0" + str(max(A))
            A.remove(max(A))
        else:
            return ""

        if max(A) < 6:
            return hour + ":" + str(max(A)) + str(min(A))
        elif min(A) < 6:
            return hour + ":" + str(min(A)) + str(max(A))
        else:
            return ""
```
