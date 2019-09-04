# 171. Excel Sheet Column Number
Given a column title as appear in an Excel sheet, return its corresponding column number.

For example:
```
    A -> 1
    B -> 2
    C -> 3
    ...
    Z -> 26
    AA -> 27
    AB -> 28
    ...
```

#### Example 1:
<pre>
<strong>Input:</strong> "A"
<strong>Output:</strong> 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "AB"
<strong>Output:</strong> 28
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "ZY"
<strong>Output:</strong> 701
</pre>

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def titleToNumber(self, s: str) -> int:
        ret = 0
        for c in s:
            ret *= 26
            ret += ord(c) - 64
        return ret
```
