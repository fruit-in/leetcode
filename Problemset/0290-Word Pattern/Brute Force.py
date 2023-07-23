class Solution:
    def wordPattern(self, pattern: str, str: str) -> bool:
        words = str.split()

        if len(pattern) != len(words):
            return False

        for i in range(len(pattern)):
            for j in range(i + 1, len(pattern)):
                if (pattern[j] == pattern[i]) != (words[j] == words[i]):
                    return False

        return True
