class Solution:
    def countVowelSubstrings(self, word: str) -> int:
        ret = 0

        for i in range(len(word)):
            aeiou = [False] * 5
            for j in range(i, len(word)):
                if word[j] not in "aeiou":
                    break

                aeiou[0] |= word[j] == 'a'
                aeiou[1] |= word[j] == 'e'
                aeiou[2] |= word[j] == 'i'
                aeiou[3] |= word[j] == 'o'
                aeiou[4] |= word[j] == 'u'

                if all(aeiou):
                    ret += 1

        return ret
