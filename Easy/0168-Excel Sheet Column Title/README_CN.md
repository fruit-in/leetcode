# 168. Excel表列名称
给定一个正整数，返回它在 Excel 表中相对应的列名称。

例如，
```
    1 -> A
    2 -> B
    3 -> C
    ...
    26 -> Z
    27 -> AA
    28 -> AB
    ...
```

#### 示例 1:
<pre>
<strong>输入:</strong> 1
<strong>输出:</strong> "A"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 28
<strong>输出:</strong> "AB"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 701
<strong>输出:</strong> "ZY"
</pre>

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def convertToTitle(self, n: int) -> str:
        ret = ""
        while n > 0:
            n -= 1
            ret = chr(n % 26 + 65) + ret
            n //= 26
        return ret
```
