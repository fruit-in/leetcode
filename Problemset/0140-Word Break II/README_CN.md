# 140. 单词拆分 II
给定一个字符串 `s` 和一个字符串字典 `wordDict` ，在字符串 `s` 中增加空格来构建一个句子，使得句子中所有的单词都在词典中。**以任意顺序** 返回所有这些可能的句子。

**注意：**词典中的同一个单词可能在分段中被重复使用多次。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
<strong>输出:</strong> ["cats and dog","cat sand dog"]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "pineapplepenapple", wordDict = ["apple","pen","applepen","pine","pineapple"]
<strong>输出:</strong> ["pine apple pen apple","pineapple pen apple","pine applepen apple"]
<strong>解释:</strong> 注意你可以重复使用字典中的单词。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
<strong>输出:</strong> []
</pre>

#### 提示:
* `1 <= s.length <= 20`
* `1 <= wordDict.length <= 1000`
* `1 <= wordDict[i].length <= 10`
* `s` 和 `wordDict[i]` 仅有小写英文字母组成
* `wordDict` 中所有字符串都 **不同**

## 题解 (Python)

### 1. 题解
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
