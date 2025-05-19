class Solution:
    def constructDistancedSequence(self, n: int) -> List[int]:
        def dfs(i: int, mask: int) -> bool:
            if i == len(a):
                return mask == (1 << (n + 1)) - 2
            if a[i] > 0:
                return dfs(i + 1, mask)

            for x in range(n, 0, -1):
                if (mask >> x) & 1 == 0 and (x == 1 or (i + x < len(a) and a[i + x] == 0)):
                    a[i] = x
                    if x > 1:
                        a[i + x] = x
                    if dfs(i + 1, mask | (1 << x)):
                        return True
                    a[i] = 0
                    if x > 1:
                        a[i + x] = 0

            return False

        a = [0] * (n * 2 - 1)
        dfs(0, 0)

        return a
