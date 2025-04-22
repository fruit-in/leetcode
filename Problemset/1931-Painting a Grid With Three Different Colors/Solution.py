class Solution:
    def colorTheGrid(self, m: int, n: int) -> int:
        patterns = list("rgb")
        nextpatterns = {}
        ways = {}

        for _ in range(m - 1):
            tmp = []

            for p in patterns:
                for c in "rgb":
                    if p[-1] != c:
                        tmp.append(p + c)

            patterns = tmp

        nextpatterns = {p: [] for p in patterns}
        ways = {p: 1 for p in patterns}

        for i in range(len(patterns)):
            for j in range(i + 1, len(patterns)):
                if all(c0 != c1 for c0, c1 in zip(patterns[i], patterns[j])):
                    nextpatterns[patterns[i]].append(patterns[j])
                    nextpatterns[patterns[j]].append(patterns[i])

        for _ in range(n - 1):
            tmp = {p: 0 for p in patterns}

            for p0 in patterns:
                for p1 in nextpatterns[p0]:
                    tmp[p1] = (tmp[p1] + ways[p0]) % 1000000007

            ways = tmp

        return sum(ways.values()) % 1000000007
