class Solution:
    def kthFactor(self, n: int, k: int) -> int:
        factor = 1
        i = 0

        while factor * factor <= n:
            if n % factor == 0:
                i += 1
                if i == k:
                    return factor
            factor += 1

        factor -= 1
        factor -= 1 if factor * factor == n else 0

        while factor > 0:
            if n % factor == 0:
                i += 1
                if i == k:
                    return n // factor
            factor -= 1

        return -1
