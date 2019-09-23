# 290. Word Pattern
Given a ```pattern``` and a string ```str```, find if ```str``` follows the same pattern.

Here **follow** means a full match, such that there is a bijection between a letter in ```pattern``` and a **non-empty** word in ```str```.

#### Example 1:
<pre>
<strong>Input:</strong> pattern = "abba", str = "dog cat cat dog"
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> pattern = "abba", str = "dog cat cat fish"
<strong>Output:</strong> false
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> pattern = "aaaa", str = "dog cat cat dog"
<strong>Output:</strong> false
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> pattern = "abba", str = "dog dog dog dog"
<strong>Output:</strong> false
</pre>

#### Note:
You may assume ```pattern``` contains only lowercase letters, and ```str``` contains lowercase letters that may be separated by a single space.

## Solutions (Python)

### 1. Brute Force
```Python3
class Solution:
    def wordPattern(self, pattern: str, str: str) -> bool:
        words = str.split()

        if len(pattern) != len(words):
            return False

        for i in range(len(pattern)):
            for j in range(i + 1, len(pattern)):
                if (pattern[j] == pattern[i]) != (words[j] == words[i]):
                    return False

        return True
```

### 2. HashMap
```Python3
class Solution:
    def wordPattern(self, pattern: str, str: str) -> bool:
        words = str.split()

        if len(pattern) != len(words):
            return False

        match = {}
        used = set()

        for ch, wo in zip(pattern, words):
            if (ch in match) != (wo in used):
                return False
            elif ch not in match:
                match[ch] = wo
                used.add(wo)
            elif match[ch] != wo:
                return False

        return True
```
