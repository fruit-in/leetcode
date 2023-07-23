class Solution:
    def canBeTypedWords(self, text: str, brokenLetters: str) -> int:
        ret = 0

        for word in text.split():
            if all(c not in brokenLetters for c in word):
                ret += 1

        return ret
