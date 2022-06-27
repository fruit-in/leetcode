# 1967. Number of Strings That Appear as Substrings in Word
Given an array of strings `patterns` and a string `word`, return *the **number** of strings in* `patterns` *that exist as a **substring** in* `word`.

A **substring** is a contiguous sequence of characters within a string.

#### Example 1:
<pre>
<strong>Input:</strong> patterns = ["a","abc","bc","d"], word = "abc"
<strong>Output:</strong> 3
<strong>Explanation:</strong>
- "a" appears as a substring in "abc".
- "abc" appears as a substring in "abc".
- "bc" appears as a substring in "abc".
- "d" does not appear as a substring in "abc".
3 of the strings in patterns appear as a substring in word.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> patterns = ["a","b","c"], word = "aaaaabbbbb"
<strong>Output:</strong> 2
<strong>Explanation:</strong>
- "a" appears as a substring in "aaaaabbbbb".
- "b" appears as a substring in "aaaaabbbbb".
- "c" does not appear as a substring in "aaaaabbbbb".
2 of the strings in patterns appear as a substring in word.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> patterns = ["a","a","a"], word = "ab"
<strong>Output:</strong> 3
<strong>Explanation:</strong> Each of the patterns appears as a substring in word "ab".
</pre>

#### Constraints:
* `1 <= patterns.length <= 100`
* `1 <= patterns[i].length <= 100`
* `1 <= word.length <= 100`
* `patterns[i]` and `word` consist of lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def numOfStrings(self, patterns: List[str], word: str) -> int:
        return len([p for p in patterns if p in word])
```
