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
