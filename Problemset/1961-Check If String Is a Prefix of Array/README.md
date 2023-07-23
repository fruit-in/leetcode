# 1961. Check If String Is a Prefix of Array
Given a string `s` and an array of strings `words`, determine whether `s` is a **prefix string** of `words`.

A string `s` is a **prefix string** of `words` if s can be made by concatenating the first `k` strings in `words` for some **positive** `k` no larger than `words.length`.

Return `true` *if* `s` *is a **prefix string** of* `words`, *or* `false` *otherwise*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "iloveleetcode", words = ["i","love","leetcode","apples"]
<strong>Output:</strong> true
<strong>Explanation:</strong>
s can be made by concatenating "i", "love", and "leetcode" together.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "iloveleetcode", words = ["apples","i","love","leetcode"]
<strong>Output:</strong> false
<strong>Explanation:</strong>
It is impossible to make s using a prefix of arr.
</pre>

#### Constraints:
* `1 <= words.length <= 100`
* `1 <= words[i].length <= 20`
* `1 <= s.length <= 1000`
* `words[i]` and `s` consist of only lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def isPrefixString(self, s: str, words: List[str]) -> bool:
        if s == "":
            return True
        elif words == []:
            return False
        elif len(s) < len(words[0]):
            return False
        elif s[:len(words[0])] != words[0]:
            return False
        else:
            return self.isPrefixString(s[len(words[0]):], words[1:])
```
