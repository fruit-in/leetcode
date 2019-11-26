# 504. 七进制数
给定一个整数，将其转化为7进制，并以字符串形式输出。

#### 示例 1:
<pre>
<strong>输入:</strong> 100
<strong>输出:</strong> "202"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> -7
<strong>输出:</strong> "-10"
</pre>

**注意:** 输入范围是 [-1e7, 1e7] 。

## 题解 (Python)

### 1. 题解
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
