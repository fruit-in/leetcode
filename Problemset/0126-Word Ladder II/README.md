# 126. Word Ladder II
A **transformation sequence** from word `beginWord` to word `endWord` using a dictionary `wordList` is a sequence of words <code>beginWord -> s<sup>1</sup> -> s<sup>2</sup> -> ... -> s<sup>k</sup></code> such that:
* Every adjacent pair of words differs by a single letter.
* Every <code>s<sup>i</sup></code> for `1 <= i <= k` is in `wordList`. Note that `beginWord` does not need to be in `wordList`.
* <code>s<sup>k</sup> == endWord</code>

Given two words, `beginWord` and `endWord`, and a dictionary `wordList`, return *all the **shortest transformation sequences** from* `beginWord` *to* `endWord`*, or an empty list if no such sequence exists. Each sequence should be returned as a list of the words* <code>[beginWord, s<sup>1</sup>, s<sup>2</sup>, ..., s<sup>k</sup>]</code>.

#### Example 1:
<pre>
<strong>Input:</strong> beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
<strong>Output:</strong> [["hit","hot","dot","dog","cog"],["hit","hot","lot","log","cog"]]
<strong>Explanation:</strong> There are 2 shortest transformation sequences:
"hit" -> "hot" -> "dot" -> "dog" -> "cog"
"hit" -> "hot" -> "lot" -> "log" -> "cog"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
<strong>Output:</strong> []
<strong>Explanation:</strong> The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.
</pre>

#### Constraints:
* `1 <= beginWord.length <= 5`
* `endWord.length == beginWord.length`
* `1 <= wordList.length <= 500`
* `wordList[i].length == beginWord.length`
* `beginWord`, `endWord`, and `wordList[i]` consist of lowercase English letters.
* `beginWord != endWord`
* All the words in `wordList` are **unique**.
* The **sum** of all shortest transformation sequences does not exceed <code>10<sup>5</sup></code>.

## Solutions (Python)

### 1. Solution
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
