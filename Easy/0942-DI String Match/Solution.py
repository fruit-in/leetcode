class Solution:
    def diStringMatch(self, S: str) -> List[int]:
        b, e = 0, len(S)
        A = []
        for di in S:
            if di == 'I':
                A.append(b)
                b += 1
            elif di == 'D':
                A.append(e)
                e -= 1
        A.append(b)
        return A
