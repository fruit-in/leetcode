class Solution:
    def findLongestWord(self, s: str, dictionary: List[str]) -> str:
        for word in sorted(dictionary, key=lambda word: (-len(word), word)):
            i = 0

            for j in range(len(s)):
                if word[i] == s[j]:
                    i += 1

                if i == len(word):
                    return word

        return ""
