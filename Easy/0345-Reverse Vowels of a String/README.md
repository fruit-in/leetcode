# 345. Reverse Vowels of a String
Write a function that takes a string as input and reverse only the vowels of a string.

#### Example 1:
<pre>
<strong>Input:</strong> "hello"
<strong>Output:</strong> "holle"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "leetcode"
<strong>Output:</strong> "leotcede"
</pre>

#### Note:
The vowels does not include the letter "y".

## Solutions (Python)

### 1. Two Pointers
```Python3
class Solution:
    def reverseVowels(self, s: str) -> str:
        s = list(s)
        vowels = "aiueoAIUEO"
        i = 0
        j = len(s) - 1
        while i < j:
            while i < j and s[i] not in vowels:
                i += 1
            while i < j and s[j] not in vowels:
                j -= 1
            s[i], s[j] = s[j], s[i]
            i += 1
            j -= 1
        return ''.join(s)
```
