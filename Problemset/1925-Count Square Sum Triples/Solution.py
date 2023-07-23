class Solution:
    def countTriples(self, n: int) -> int:
        ret = 0

        for a in range(1, n):
            for b in range(1, n):
                c = int((a * a + b * b) ** 0.5)
                if c <= n and c * c == a * a + b * b:
                    ret += 1

        return ret
