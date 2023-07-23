class Solution:
    def isAlienSorted(self, words: List[str], order: str) -> bool:
        alien_order = [0] * 26
        for i in range(len(order)):
            alien_order[ord(order[i]) - 97] = i

        for i in range(len(words) - 1):
            for j in range(len(words[i])):
                if (j == len(words[i + 1]) or
                    alien_order[ord(words[i][j]) - 97] >
                    alien_order[ord(words[i + 1][j]) - 97]):
                    return False
                elif (alien_order[ord(words[i][j]) - 97] <
                      alien_order[ord(words[i + 1][j]) - 97]):
                    break

        return True
