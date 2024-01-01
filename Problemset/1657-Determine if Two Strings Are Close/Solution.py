class Solution:
    def closeStrings(self, word1: str, word2: str) -> bool:
        if len(word1) != len(word2) or set(word1) != set(word2):
            return False

        count1 = [0] * 26
        count2 = [0] * 26

        for ch1, ch2 in zip(word1, word2):
            count1[ord(ch1) - 97] += 1
            count2[ord(ch2) - 97] += 1

        return sorted(count1) == sorted(count2)
