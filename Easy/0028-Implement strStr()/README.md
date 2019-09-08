# 28. Implement strStr()
Implement [strStr()](http://www.cplusplus.com/reference/cstring/strstr/).

Return the index of the first occurrence of needle in haystack, or **-1** if needle is not part of haystack.

#### Example 1:
<pre>
<strong>Input:</strong> haystack = "hello", needle = "ll"
<strong>Output:</strong> 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> haystack = "aaaaa", needle = "bba"
<strong>Output:</strong> -1
</pre>

#### Clarification:
What should we return when ```needle``` is an empty string? This is a great question to ask during an interview.

For the purpose of this problem, we will return 0 when ```needle``` is an empty string. This is consistent to C's [strstr()](http://www.cplusplus.com/reference/cstring/strstr/) and Java's [indexOf()](https://docs.oracle.com/javase/7/docs/api/java/lang/String.html#indexOf(java.lang.String)).

## Solutions (Python)

### 1. Linear Scan
```Python3
class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        if not needle:
            return 0
        for i in range(len(haystack) - len(needle) + 1):
            flag = True
            for j in range(0, len(needle)):
                if haystack[i + j] != needle[j]:
                    flag = False
                    break
            if flag:
                return i
        return -1
```
