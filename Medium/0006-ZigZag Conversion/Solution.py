class Solution:
    def convert(self, s: str, numRows: int) -> str:
        rows = [[] for _ in range(numRows)]
        i = 0
        increase = True

        for ch in s:
            rows[i].append(ch)

            i += 1 if increase else -1
            if i == numRows:
                i -= 2
                increase = False
            if i <= 0:
                increase = True

        return ''.join(''.join(r) for r in rows)
