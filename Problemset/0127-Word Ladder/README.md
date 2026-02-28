# 127. Word Ladder
A **transformation sequence** from word `beginWord` to word `endWord` using a dictionary `wordList` is a sequence of words <code>beginWord -> s<sub>1</sub> -> s<sub>2</sub> -> ... -> s<sub>k</sub></code> such that:
* Every adjacent pair of words differs by a single letter.
* Every <code>s<sub>i</sub></code> for `1 <= i <= k` is in `wordList`. Note that `beginWord` does not need to be in `wordList`.
* <code>s<sub>k</sub> == endWord</code>

Given two words, `beginWord` and `endWord`, and a dictionary `wordList`, return *the **number of words** in the **shortest transformation sequence** from* `beginWord` *to* `endWord`*, or* `0` *if no such sequence exists*.

#### Example 1:
<pre>
<strong>Input:</strong> beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
<strong>Output:</strong> 5
<strong>Explanation:</strong> One shortest transformation sequence is "hit" -> "hot" -> "dot" -> "dog" -> cog", which is 5 words long.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.
</pre>

#### Constraints:
* `1 <= beginWord.length <= 10`
* `endWord.length == beginWord.length`
* `1 <= wordList.length <= 5000`
* `wordList[i].length == beginWord.length`
* `beginWord`, `endWord`, and `wordList[i]` consist of lowercase English letters.
* `beginWord != endWord`
* All the words in `wordList` are **unique**.

## Solutions (Python)

### 1. Solution
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
