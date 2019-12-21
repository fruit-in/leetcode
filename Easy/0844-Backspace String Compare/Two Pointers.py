class Solution:
    def backspaceCompare(self, S: str, T: str) -> bool:
        i, j = len(S) - 1, len(T) - 1

        while i >= 0 or j >= 0:
            cnt = 0
            while i >= 0 and (S[i] == '#' or cnt > 0):
                cnt += 1 if S[i] == '#' else -1
                i -= 1

            cnt = 0
            while j >= 0 and (T[j] == '#' or cnt > 0):
                cnt += 1 if T[j] == '#' else -1
                j -= 1

            if (i < 0) != (j < 0) or (i + j >= 0 and S[i] != T[j]):
                return False

            i -= 1
            j -= 1

        return True
