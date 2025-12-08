# 472. Concatenated Words
Given an array of strings `words` (**without duplicates**), return *all the **concatenated words** in the given list of* `words`.

A **concatenated word** is defined as a string that is comprised entirely of at least two shorter words (not necessarily distinct) in the given array.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["cat","cats","catsdogcats","dog","dogcatsdog","hippopotamuses","rat","ratcatdogcat"]
<strong>Output:</strong> ["catsdogcats","dogcatsdog","ratcatdogcat"]
<strong>Explanation:</strong> "catsdogcats" can be concatenated by "cats", "dog" and "cats";
"dogcatsdog" can be concatenated by "dog", "cats" and "dog";
"ratcatdogcat" can be concatenated by "rat", "cat", "dog" and "cat".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["cat","dog","catdog"]
<strong>Output:</strong> ["catdog"]
</pre>

#### Constraints:
* <code>1 <= words.length <= 10<sup>4</sup></code>
* `1 <= words[i].length <= 30`
* `words[i]` consists of only lowercase English letters.
* All the strings of `words` are **unique**.
* <code>1 <= sum(words[i].length) <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
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
