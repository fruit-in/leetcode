# 140. Word Break II
Given a string `s` and a dictionary of strings `wordDict`, add spaces in `s` to construct a sentence where each word is a valid dictionary word. Return all such possible sentences in **any order**.

**Note** that the same word in the dictionary may be reused multiple times in the segmentation.

#### Example 1:
<pre>
<strong>Input:</strong> s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
<strong>Output:</strong> ["cats and dog","cat sand dog"]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "pineapplepenapple", wordDict = ["apple","pen","applepen","pine","pineapple"]
<strong>Output:</strong> ["pine apple pen apple","pineapple pen apple","pine applepen apple"]
<strong>Explanation:</strong> Note that you are allowed to reuse a dictionary word.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
<strong>Output:</strong> []
</pre>

#### Constraints:
* `1 <= s.length <= 20`
* `1 <= wordDict.length <= 1000`
* `1 <= wordDict[i].length <= 10`
* `s` and `wordDict[i]` consist of only lowercase English letters.
* All the strings of `wordDict` are **unique**.
* Input is generated in a way that the length of the answer doesn't exceed 10<sup>5</sup>.

## Solutions (Python)

### 1. Solution
```Python
from functools import cache


class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> List[str]:
        words = set(wordDict)

        @cache
        def backtracking(s: str) -> Optional[List[str]]:
            ret = []

            for i in range(1, min(len(s) + 1, 10)):
                if s[:i] in wordDict:
                    if i == len(s):
                        return ret + [s]
                    sentences = backtracking(s[i:])
                    if sentences is not None:
                        ret.extend("{} {}".format(s[:i], sentence)
                                   for sentence in sentences)

            return ret if ret != [] else None

        return backtracking(s) if backtracking(s) is not None else []
```
