# """
# This is Master's API interface.
# You should not implement it, or speculate about its implementation
# """
# class Master:
#     def guess(self, word: str) -> int:

class Solution:
    def findSecretWord(self, words: List[str], master: 'Master') -> None:
        while words:
            buckets = {}
            minmax = inf
            word = words[0]

            for word0 in words:
                buckets[word0] = [[] for _ in range(7)]
                for word1 in words:
                    if word1 != word0:
                        buckets[word0][sum(word0[i] == word1[i]
                                           for i in range(6))].append(word1)

                maxcount = max(len(buckets[word0][i]) for i in range(6))
                if maxcount < minmax:
                    minmax = maxcount
                    word = word0

            matches = master.guess(word)
            words = buckets[word][matches]
