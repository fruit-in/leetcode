class Solution:
    def shortestToChar(self, S: str, C: str) -> List[int]:
        prev = -len(S)
        next = S.find(C)
        ret = []

        for i in range(len(S)):
            ret.append(min(i - prev, next - i))

            if i == next:
                prev = next
                try:
                    next = S.index(C, next + 1)
                except:
                    next = 2 * len(S)

        return ret
