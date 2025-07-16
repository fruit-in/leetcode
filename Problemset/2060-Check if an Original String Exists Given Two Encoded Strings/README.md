# 2060. Check if an Original String Exists Given Two Encoded Strings
An original string, consisting of lowercase English letters, can be encoded by the following steps:
* Arbitrarily **split** it into a **sequence** of some number of **non-empty** substrings.
* Arbitrarily choose some elements (possibly none) of the sequence, and **replace** each with **its length** (as a numeric string).
* **Concatenate** the sequence as the encoded string.

For example, **one way** to encode an original string `"abcdefghijklmnop"` might be:
* Split it as a sequence: `["ab", "cdefghijklmn", "o", "p"]`.
* Choose the second and third elements to be replaced by their lengths, respectively. The sequence becomes `["ab", "12", "1", "p"]`.
* Concatenate the elements of the sequence to get the encoded string: `"ab121p"`.

Given two encoded strings `s1` and `s2`, consisting of lowercase English letters and digits `1-9` (inclusive), return `true` *if there exists an original string that could be encoded as **both*** `s1` *and* `s2`. *Otherwise, return* `false`.

**Note:** The test cases are generated such that the number of consecutive digits in `s1` and `s2` does not exceed `3`.

#### Example 1:
<pre>
<strong>Input:</strong> s1 = "internationalization", s2 = "i18n"
<strong>Output:</strong> true
<strong>Explanation:</strong> It is possible that "internationalization" was the original string.
- "internationalization"
  -> Split:       ["internationalization"]
  -> Do not replace any element
  -> Concatenate:  "internationalization", which is s1.
- "internationalization"
  -> Split:       ["i", "nternationalizatio", "n"]
  -> Replace:     ["i", "18",                 "n"]
  -> Concatenate:  "i18n", which is s2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s1 = "l123e", s2 = "44"
<strong>Output:</strong> true
<strong>Explanation:</strong> It is possible that "leetcode" was the original string.
- "leetcode"
  -> Split:      ["l", "e", "et", "cod", "e"]
  -> Replace:    ["l", "1", "2",  "3",   "e"]
  -> Concatenate: "l123e", which is s1.
- "leetcode"
  -> Split:      ["leet", "code"]
  -> Replace:    ["4",    "4"]
  -> Concatenate: "44", which is s2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s1 = "a5b", s2 = "c5b"
<strong>Output:</strong> false
<strong>Explanation:</strong> It is impossible.
- The original string encoded as s1 must start with the letter 'a'.
- The original string encoded as s2 must start with the letter 'c'.
</pre>

#### Constraints:
* `1 <= s1.length, s2.length <= 40`
* `s1` and `s2` consist of digits `1-9` (inclusive), and lowercase English letters only.
* The number of consecutive digits in `s1` and `s2` does not exceed `3`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def possiblyEquals(self, s1: str, s2: str) -> bool:
        @cache
        def isMatchWithSkips(i: int, j: int, diff: int) -> bool:
            if i == len(s1) and j == len(s2):
                return diff == 0
            elif j == len(s2) or diff < 0:
                if diff >= 0 or i == len(s1):
                    return False
                elif s1[i].islower():
                    return isMatchWithSkips(i + 1, j, diff + 1)
                else:
                    for k in range(3):
                        if i + k < len(s1) and s1[i:i + k + 1].isdigit() and isMatchWithSkips(i + k + 1, j, diff + int(s1[i:i + k + 1])):
                            return True
                    return False
            elif i == len(s1) or diff > 0:
                if diff <= 0 or j == len(s2):
                    return False
                elif s2[j].islower():
                    return isMatchWithSkips(i, j + 1, diff - 1)
                else:
                    for k in range(3):
                        if j + k < len(s2) and s2[j:j + k + 1].isdigit() and isMatchWithSkips(i, j + k + 1, diff - int(s2[j:j + k + 1])):
                            return True
                    return False
            else:
                if s1[i].islower() and s2[j].islower():
                    return s1[i] == s2[j] and isMatchWithSkips(i + 1, j + 1, diff)
                elif s1[i].isdigit():
                    for k in range(3):
                        if i + k < len(s1) and s1[i:i + k + 1].isdigit() and isMatchWithSkips(i + k + 1, j, diff + int(s1[i:i + k + 1])):
                            return True
                    return False
                else:
                    for k in range(3):
                        if j + k < len(s2) and s2[j:j + k + 1].isdigit() and isMatchWithSkips(i, j + k + 1, diff - int(s2[j:j + k + 1])):
                            return True
                    return False

        return isMatchWithSkips(0, 0, 0)
```
