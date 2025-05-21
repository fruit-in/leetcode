# 126. 单词接龙 II
按字典 `wordList` 完成从单词 `beginWord` 到单词 `endWord` 转化，一个表示此过程的 **转换序列** 是形式上像 <code>beginWord -> s<sup>1</sup> -> s<sup>2</sup> -> ... -> s<sup>k</sup></code> 这样的单词序列，并满足：
* 每对相邻的单词之间仅有单个字母不同。
* 转换过程中的每个单词 <code>s<sup>i</sup></code>（`1 <= i <= k`）必须是字典 `wordList` 中的单词。注意，`beginWord` 不必是字典 `wordList` 中的单词。
* <code>s<sup>k</sup> == endWord</code>

给你两个单词 `beginWord` 和 `endWord` ，以及一个字典 `wordList` 。请你找出并返回所有从 `beginWord` 到 `endWord` 的 **最短转换序列** ，如果不存在这样的转换序列，返回一个空列表。每个序列都应该以单词列表 <code>[beginWord, s<sup>1</sup>, s<sup>2</sup>, ..., s<sup>k</sup>]</code> 的形式返回。

#### 示例 1:
<pre>
<strong>输入:</strong> beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
<strong>输出:</strong> [["hit","hot","dot","dog","cog"],["hit","hot","lot","log","cog"]]
<strong>解释:</strong> 存在 2 种最短的转换序列：
"hit" -> "hot" -> "dot" -> "dog" -> "cog"
"hit" -> "hot" -> "lot" -> "log" -> "cog"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
<strong>输出:</strong> []
<strong>解释:</strong> endWord "cog" 不在字典 wordList 中，所以不存在符合要求的转换序列。
</pre>

#### 提示:
* `1 <= beginWord.length <= 5`
* `endWord.length == beginWord.length`
* `1 <= wordList.length <= 500`
* `wordList[i].length == beginWord.length`
* `beginWord`、`endWord` 和 `wordList[i]` 由小写英文字母组成
* `beginWord != endWord`
* All the words in `wordList` are **unique**.
* `wordList` 中的所有单词 **互不相同**

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def findLadders(self, beginWord: str, endWord: str, wordList: List[str]) -> List[List[str]]:
        def dfs(i: int) -> None:
            if i == len(path):
                if path[-1] == endWord:
                    ret.append(path.copy())
                return

            for word in neighbors[path[i - 1]]:
                if word not in visited and shortestb.get(word, 0) == i + 1:
                    path[i] = word
                    visited.add(word)
                    dfs(i + 1)
                    visited.remove(word)

        wordList.extend([beginWord] if beginWord not in wordList else [])
        neighbors = {word: [] for word in wordList}
        ret = []

        for i in range(len(wordList)):
            for j in range(i + 1, len(wordList)):
                diff = 0

                for c0, c1 in zip(wordList[i], wordList[j]):
                    if c0 != c1:
                        diff += 1
                    if diff > 1:
                        break

                if diff < 2:
                    neighbors[wordList[i]].append(wordList[j])
                    neighbors[wordList[j]].append(wordList[i])

        if neighbors[beginWord] == [] or neighbors.get(endWord, []) == []:
            return []

        queue = deque([beginWord])
        shortestb = {beginWord: 1}
        minlen = inf

        while queue:
            word0 = queue.popleft()
            length = shortestb[word0]

            if word0 == endWord:
                minlen = length
                shortestb = {word: length for word, length in shortestb.items() if length < minlen} | {
                    endWord: minlen}
                break

            for word1 in neighbors[word0]:
                if word1 not in shortestb:
                    queue.append(word1)
                    shortestb[word1] = length + 1

        if minlen == inf:
            return []

        queue = deque([endWord])
        shorteste = {endWord: 1}

        while queue:
            word0 = queue.popleft()
            length = shorteste[word0]

            if word0 == beginWord:
                shorteste = {word: length for word, length in shorteste.items() if length < minlen} | {
                    beginWord: minlen}
                break

            for word1 in neighbors[word0]:
                if word1 not in shorteste:
                    queue.append(word1)
                    shorteste[word1] = length + 1

        neighbors = {word: neighbor for word, neighbor in neighbors.items(
        ) if shortestb.get(word, -inf) + shorteste.get(word, -inf) == minlen + 1}
        shortestb = {word: length for word,
                     length in shortestb.items() if word in neighbors}
        path = [""] * minlen
        path[0] = beginWord
        visited = {beginWord}
        dfs(1)

        return ret
```
