# 6. ZigZag Conversion
The string ```"PAYPALISHIRING"``` is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
```
P   A   H   N
A P L S I I G
Y   I   R
```

And then read line by line: ```"PAHNAPLSIIGYIR"```

Write the code that will take a string and make this conversion given a number of rows:
```
string convert(string s, int numRows);
```

#### Example 1:
<pre>
<strong>Input:</strong> s = "PAYPALISHIRING", numRows = 3
<strong>Output:</strong> "PAHNAPLSIIGYIR"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "PAYPALISHIRING", numRows = 4
<strong>Output:</strong> "PINALSIGYAHRPI"
<strong>Explanation:</strong>
P     I    N
A   L S  I G
Y A   H R
P     I
</pre>

## Solutions (Python)

### 1. Solution
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
