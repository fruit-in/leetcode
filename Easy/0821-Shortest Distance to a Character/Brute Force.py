class Solution:
    def shortestToChar(self, S: str, C: str) -> List[int]:
        ret = []

        for i in range(len(S)):
            shortest = len(S)

            for j in range(len(S)):
                if S[j] == C:
                    shortest = min(shortest, abs(i - j))

            ret.append(shortest)

        return ret
