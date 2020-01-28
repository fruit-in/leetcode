# 6. Z 字形变换
将一个给定字符串根据给定的行数，以从上往下、从左到右进行 Z 字形排列。

比如输入字符串为 ```"LEETCODEISHIRING"``` 行数为 3 时，排列如下：
```
L   C   I   R
E T O E S I I G
E   D   H   N
```

之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如：```"LCIRETOESIIGEDHN"```。

请你实现这个将字符串进行指定行数变换的函数：
```string convert(string s, int numRows);```

#### 示例 1:
<pre>
<strong>输入:</strong> s = "LEETCODEISHIRING", numRows = 3
<strong>输出:</strong> "LEETCODEISHIRING"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "LEETCODEISHIRING", numRows = 4
<strong>输出:</strong> "LEETCODEISHIRING"
<strong>解释:</strong>
L     D     R
E   O E   I I
E C   I H   N
T     S     G
</pre>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def convert(self, s: str, numRows: int) -> str:
        rows = [[] for _ in range(numRows)]
        i = 0
        increase = True

        for ch in s:
            rows[i].append(ch)

            i += 1 if increase else -1
            if i == numRows:
                i -= 2
                increase = False
            if i <= 0:
                increase = True

        return ''.join(''.join(r) for r in rows)
```
