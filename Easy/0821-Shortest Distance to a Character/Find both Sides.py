class Solution:
    def shortestToChar(self, S: str, C: str) -> List[int]:
        ret = []

        for i in range(len(S)):
            for j in range(len(S)):
                l_char = S[i - j] if i - j >= 0 else ''
                r_char = S[i + j] if i + j < len(S) else ''

                if l_char == C or r_char == C:
                    ret.append(j)
                    break

        return ret
