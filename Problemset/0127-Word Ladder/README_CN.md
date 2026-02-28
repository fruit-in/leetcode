# 127. 单词接龙
字典 `wordList` 中从单词 `beginWord` 到 `endWord` 的 **转换序列** 是一个按下述规格形成的序列 <code>beginWord -> s<sub>1</sub> -> s<sub>2</sub> -> ... -> s<sub>k</sub></code>：
* 每一对相邻的单词只差一个字母。
* 对于 `1 <= i <= k` 时，每个 <code>s<sub>i</sub></code> 都在 `wordList` 中。注意， `beginWord` 不需要在 `wordList` 中。
* <code>s<sub>k</sub> == endWord</code>

给你两个单词 `beginWord` 和 `endWord` 和一个字典 `wordList` ，返回 *从 `beginWord` 到 `endWord` 的 **最短转换序列** 中的 **单词数目*** 。如果不存在这样的转换序列，返回 `0` 。

#### 示例 1:
<pre>
<strong>输入:</strong> beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
<strong>输出:</strong> 5
<strong>解释:</strong> 一个最短转换序列是 "hit" -> "hot" -> "dot" -> "dog" -> "cog", 返回它的长度 5。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
<strong>输出:</strong> 0
<strong>解释:</strong> endWord "cog" 不在字典中，所以无法进行转换。
</pre>

#### 提示:
* `1 <= beginWord.length <= 10`
* `endWord.length == beginWord.length`
* `1 <= wordList.length <= 5000`
* `wordList[i].length == beginWord.length`
* `beginWord`、`endWord` 和 `wordList[i]` 由小写英文字母组成
* `beginWord != endWord`
* `wordList` 中的所有字符串 **互不相同**

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        patterns = {word: [] for word in wordList}
        pattern2words = {}
        queue = deque([beginWord])
        visited = set()
        minstep = {beginWord: 1}

        if beginWord not in patterns:
            wordList.append(beginWord)
            patterns[beginWord] = []

        for word in wordList:
            chars = list(word)
            for i in range(len(chars)):
                chars[i] = '.'
                pattern = ''.join(chars)
                if pattern not in pattern2words:
                    pattern2words[pattern] = []
                pattern2words[pattern].append(word)
                patterns[word].append(pattern)
                chars[i] = word[i]

        while queue and queue[0] != endWord:
            word = queue.popleft()
            for pattern in patterns[word]:
                if pattern not in visited:
                    visited.add(pattern)
                    for newword in pattern2words[pattern]:
                        if newword not in minstep:
                            minstep[newword] = minstep[word] + 1
                            queue.append(newword)

        return minstep.get(endWord, 0)
```
