# 472. 连接词
给你一个 **不含重复** 单词的字符串数组 `words` ，请你找出并返回 `words` 中的所有 **连接词** 。

**连接词** 定义为：一个完全由给定数组中的至少两个较短单词（不一定是不同的两个单词）组成的字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["cat","cats","catsdogcats","dog","dogcatsdog","hippopotamuses","rat","ratcatdogcat"]
<strong>输出:</strong> ["catsdogcats","dogcatsdog","ratcatdogcat"]
<strong>解释:</strong> "catsdogcats" 由 "cats", "dog" 和 "cats" 组成;
     "dogcatsdog" 由 "dog", "cats" 和 "dog" 组成;
     "ratcatdogcat" 由 "rat", "cat", "dog" 和 "cat" 组成。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["cat","dog","catdog"]
<strong>输出:</strong> ["catdog"]
</pre>

#### 提示:
* <code>1 <= words.length <= 10<sup>4</sup></code>
* `1 <= words[i].length <= 30`
* `words[i]` 仅由小写英文字母组成。
* `words` 中的所有字符串都是 **唯一** 的。
* <code>1 <= sum(words[i].length) <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class TrieNode:
    def __init__(self):
        self.children = {}
        self.isend = False


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        curr = self.root

        for i in range(len(word)):
            if word[i] not in curr.children:
                curr.children[word[i]] = TrieNode()
            curr = curr.children[word[i]]

        curr.isend = True

    def search(self, word: str) -> bool:
        n = len(word)
        dp = [False] * (n + 1)
        dp[0] = True

        for i in range(n):
            if not dp[i]:
                continue

            curr = self.root

            for j in range(i, n):
                if word[j] not in curr.children:
                    break
                curr = curr.children[word[j]]
                dp[j + 1] |= curr.isend

        return dp[n]


class Solution:
    def findAllConcatenatedWordsInADict(self, words: List[str]) -> List[str]:
        trie = Trie()
        ret = []

        for word in sorted(words, key=len):
            if trie.search(word):
                ret.append(word)
            else:
                trie.insert(word)

        return ret
```
