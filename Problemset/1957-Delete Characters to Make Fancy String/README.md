# 1957. Delete Characters to Make Fancy String
A **fancy string** is a string where no **three consecutive** characters are equal.

Given a string `s`, delete the **minimum** possible number of characters from `s` to make it **fancy**.

Return *the final string after the deletion*. It can be shown that the answer will always be **unique**.

#### Example 1:
<pre>
<strong>Input:</strong> s = "leeetcode"
<strong>Output:</strong> "leetcode"
<strong>Explanation:</strong>
Remove an 'e' from the first group of 'e's to create "leetcode".
No three consecutive characters are equal, so return "leetcode".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aaabaaaa"
<strong>Output:</strong> "aabaa"
<strong>Explanation:</strong>
Remove an 'a' from the first group of 'a's to create "aabaaaa".
Remove two 'a's from the second group of 'a's to create "aabaa".
No three consecutive characters are equal, so return "aabaa".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "aab"
<strong>Output:</strong> "aab"
<strong>Explanation:</strong> No three consecutive characters are equal, so return "aab".
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists only of lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def makeFancyString(self, s: str) -> str:
        ret = ""

        for c in s:
            if len(ret) < 2 or ret[-1] != c or ret[-2] != c:
                ret += c

        return ret
```

## Solutions (Language)

### 1. Solution
```Language
```
