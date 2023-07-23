# 524. Longest Word in Dictionary through Deleting
Given a string `s` and a string array `dictionary`, return *the longest string in the dictionary that can be formed by deleting some of the given string characters*. If there is more than one possible result, return the longest word with the smallest lexicographical order. If there is no possible result, return the empty string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abpcplea", dictionary = ["ale","apple","monkey","plea"]
<strong>Output:</strong> "apple"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abpcplea", dictionary = ["a","b","c"]
<strong>Output:</strong> "a"
</pre>

#### Constraints:
* `1 <= s.length <= 1000`
* `1 <= dictionary.length <= 1000`
* `1 <= dictionary[i].length <= 1000`
* `s` and `dictionary[i]` consist of lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def findLongestWord(self, s: str, dictionary: List[str]) -> str:
        for word in sorted(dictionary, key=lambda word: (-len(word), word)):
            i = 0

            for j in range(len(s)):
                if word[i] == s[j]:
                    i += 1

                if i == len(word):
                    return word

        return ""
```
