class Solution:
    @cache
    def superEggDrop(self, k: int, n: int) -> int:
        if n == 0:
            return 0
        if k == 1:
            return n

        x = bisect_right(range(1, n + 1), 0, key=lambda x: self.superEggDrop(k -
                         1, x - 1) - self.superEggDrop(k, n - x)) + 1
        ret = 1 + self.superEggDrop(k - 1, x - 1)
        if x > 0:
            ret = min(ret, 1 + self.superEggDrop(k, n - x + 1))

        return ret
