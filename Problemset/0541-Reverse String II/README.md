# 541. Reverse String II
Given a string and an integer k, you need to reverse the first k characters for every 2k characters counting from the start of the string. If there are less than k characters left, reverse all of them. If there are less than 2k but greater than or equal to k characters, then reverse the first k characters and left the other as original.

#### Example:
<pre>
<strong>Input:</strong> s = "abcdefg", k = 2
<strong>Output:</strong> "bacdfeg"
</pre>

#### Restrictions:
1. The string consists of lower English letters only.
2. Length of the given string and k will in the range [1, 10000]

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def reverseStr(self, s: str, k: int) -> str:
        s_list = list(s)

        for i in range(0, len(s), 2 * k):
            j = min(i + k - 1, len(s) - 1)
            while i < j:
                s_list[i], s_list[j] = s_list[j], s_list[i]
                i += 1
                j -= 1

        return ''.join(s_list)
```
