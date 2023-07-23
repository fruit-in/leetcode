# 434. Number of Segments in a String
Count the number of segments in a string, where a segment is defined to be a contiguous sequence of non-space characters.

Please note that the string does not contain any **non-printable** characters.

#### Example:
<pre>
<strong>Input:</strong> "Hello, my name is John"
<strong>Output:</strong> 5
</pre>

## Solutions (Python)

### 1. Count
```Python3
class Solution:
    def countSegments(self, s: str) -> int:
        cnt = 0

        for i in range(len(s)):
            if not s[i].isspace() and (i == 0 or s[i - 1].isspace()):
                cnt += 1

        return cnt
```
