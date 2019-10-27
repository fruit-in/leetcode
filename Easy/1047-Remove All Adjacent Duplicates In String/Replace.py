class Solution:
    def removeDuplicates(self, S: str) -> str:
        i = 0
        while i + 1 < len(S):
            if S[i] == S[i + 1]:
                S = S.replace(S[i] * 2, '')
                i -= 1 if i > 0 else 0
            else:
                i += 1

        return S
