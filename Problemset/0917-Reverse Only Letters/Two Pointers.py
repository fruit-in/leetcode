class Solution:
    def reverseOnlyLetters(self, S: str) -> str:
        S = list(S)
        i, j = 0, len(S) - 1

        while i < j:
            if S[i].isalpha() and S[j].isalpha():
                S[i], S[j] = S[j], S[i]
                i += 1
                j -= 1
            elif not S[i].isalpha():
                i += 1
            elif not S[j].isalpha():
                j -= 1

        return ''.join(S)
