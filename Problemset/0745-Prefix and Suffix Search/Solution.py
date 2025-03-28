class WordFilter:

    def __init__(self, words: List[str]):
        self.hashmap = {}

        for i in range(len(words)):
            prefs = [words[i][:j + 1] for j in range(len(words[i]))]
            suffs = [words[i][-j - 1:] for j in range(len(words[i]))]

            for pref in prefs:
                for suff in suffs:
                    self.hashmap[(pref, suff)] = i

    def f(self, pref: str, suff: str) -> int:
        return self.hashmap.get((pref, suff), -1)


# Your WordFilter object will be instantiated and called as such:
# obj = WordFilter(words)
# param_1 = obj.f(pref,suff)
