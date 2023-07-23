class Solution:
    def countValidWords(self, sentence: str) -> int:
        return sum(1 for word in sentence.split() if self.is_valid(word))

    def is_valid(self, word: str) -> bool:
        no_hyphen = True

        for i in range(len(word)):
            if word[i].isdigit():
                return False
            elif word[i] == '-':
                if not no_hyphen:
                    return False
                elif i == 0 or i == len(word) - 1:
                    return False
                elif not (word[i - 1].islower() and word[i + 1].islower()):
                    return False
                else:
                    no_hyphen = False
            elif word[i] in "!.," and i != len(word) - 1:
                return False

        return True
