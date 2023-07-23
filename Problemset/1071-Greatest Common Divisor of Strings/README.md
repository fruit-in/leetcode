# 1071. Greatest Common Divisor of Strings
For strings <code>S</code> and <code>T</code>, we say "<code>T</code> divides <code>S</code>" if and only if <code>S = T + ... + T</code>  (<code>T</code> concatenated with itself 1 or more times)

Return the largest string <code>X</code> such that <code>X</code> divides str1 and <code>X</code> divides str2.

#### Example 1:
<pre>
<strong>Input:</strong> str1 = "ABCABC", str2 = "ABC"
<strong>Output:</strong> "ABC"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> str1 = "ABABAB", str2 = "ABAB"
<strong>Output:</strong> "AB"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> str1 = "LEET", str2 = "CODE"
<strong>Output:</strong> ""
</pre>

#### Note:
1. <code>1 <= str1.length <= 1000</code>
2. <code>1 <= str2.length <= 1000</code>
3. <code>str1[i]</code> and <code>str2[i]</code> are English uppercase letters.

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def gcdOfStrings(self, str1: str, str2: str) -> str:
        if str1 == str2:
            return str1
        if str1.startswith(str2):
            return self.gcdOfStrings(str1[len(str2):], str2)
        elif str2.startswith(str1):
            return self.gcdOfStrings(str1, str2[len(str1):])
        return ""
```
