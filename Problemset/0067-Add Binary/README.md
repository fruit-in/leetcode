# 67. Add Binary
Given two binary strings, return their sum (also a binary string).

The input strings are both **non-empty** and contains only characters ```1``` or ```0```.

#### Example 1:
<pre>
<strong>Input:</strong> a = "11", b = "1"
<strong>Output:</strong> "100"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> a = "1010", b = "1011"
<strong>Output:</strong> "10101"
</pre>

## Solutions (Python)

### 1. Solution
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
