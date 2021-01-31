class Solution:
    def maxRepeating(self, sequence: str, word: str) -> int:
        i = 0
        k = 0

        while i + (k + 1) * len(word) <= len(sequence):
            if sequence[i:i + (k + 1) * len(word)] == (k + 1) * word:
                k += 1
            else:
                i += 1

        return k
