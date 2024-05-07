class Solution:
    def longestStrChain(self, words: List[str]) -> int:
        chainlen = {word: 1 for word in words}

        for word in sorted(words, key=len):
            for i in range(len(word)):
                pred = word[:i] + word[i + 1:]
                chainlen[word] = max(chainlen[word], chainlen.get(pred, 0) + 1)

        return max(chainlen.values())
