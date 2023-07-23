class WordDictionary:

    def __init__(self):
        """
        Initialize your data structure here.
        """
        self.children = [None] * 26
        self.is_end = False

    def addWord(self, word: str) -> None:
        """
        Adds a word into the data structure.
        """
        i = ord(word[0]) - 97

        if not self.children[i]:
            self.children[i] = WordDictionary()

        if len(word) == 1:
            self.children[i].is_end = True
        else:
            self.children[i].addWord(word[1:])

    def search(self, word: str) -> bool:
        """
        Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter.
        """
        i = ord(word[0]) - 97

        if word == '.':
            return any(child.is_end for child in self.children if child)
        elif len(word) == 1:
            return self.children[i] and self.children[i].is_end
        elif word[0] == '.':
            return any(child.search(word[1:]) for child in self.children if child)
        else:
            return self.children[i] and self.children[i].search(word[1:])


# Your WordDictionary object will be instantiated and called as such:
# obj = WordDictionary()
# obj.addWord(word)
# param_2 = obj.search(word)
