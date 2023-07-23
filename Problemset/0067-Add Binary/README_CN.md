# 67. 二进制求和
给定两个二进制字符串，返回他们的和（用二进制表示）。

输入为**非空**字符串且只包含数字 ```1``` 和 ```0```。

#### 示例 1:
<pre>
<strong>输入:</strong> a = "11", b = "1"
<strong>输出:</strong> "100"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> a = "1010", b = "1011"
<strong>输出:</strong> "10101"
</pre>

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def addBinary(self, a: str, b: str) -> str:
        i = -1
        c = '0'
        ret = ""
        while i >= -len(a) or i >= -len(b):
            if (i < -len(a) or a[i] == '0') and (i < -len(b) or b[i] == '0'):
                ret = c + ret
                c = '0'
            elif i >= -len(a) and a[i] == '1' and i >= -len(b) and b[i] == '1':
                ret = c + ret
                c = '1'
            elif c == '0':
                ret = '1' + ret
            else:
                ret = '0' + ret
            i -= 1
        if c == '1':
            ret = '1' + ret
        return ret
```
