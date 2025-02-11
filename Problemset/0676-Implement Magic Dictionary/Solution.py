class MagicDictionary:

    def __init__(self):
        self.presufban = {}

    def buildDict(self, dictionary: List[str]) -> None:
        for word in dictionary:
            for i in range(len(word)):
                prefix = word[:i]
                suffix = word[i + 1:]

                if (prefix, suffix) not in self.presufban:
                    self.presufban[(prefix, suffix)] = word[i]
                else:
                    self.presufban[(prefix, suffix)] = '?'

    def search(self, searchWord: str) -> bool:
        for i in range(len(searchWord)):
            prefix = searchWord[:i]
            suffix = searchWord[i + 1:]

            if self.presufban.get((prefix, suffix), searchWord[i]) != searchWord[i]:
                return True

        return False


# Your MagicDictionary object will be instantiated and called as such:
# obj = MagicDictionary()
# obj.buildDict(dictionary)
# param_2 = obj.search(searchWord)
