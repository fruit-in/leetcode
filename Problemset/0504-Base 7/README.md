# 504. Base 7
Given an integer, return its base 7 string representation.

#### Example 1:
<pre>
<strong>Input:</strong> 100
<strong>Output:</strong> "202"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> -7
<strong>Output:</strong> "-10"
</pre>

**Note:** The input will be in range of [-1e7, 1e7]. 

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def convertToBase7(self, num: int) -> str:
        if num >= 7:
            return self.convertToBase7(num // 7) + str(num % 7)
        elif num < 0:
            return '-' + self.convertToBase7(-num)
        elif num < 7:
            return str(num)
```
