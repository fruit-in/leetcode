class Solution:
    def countEven(self, num: int) -> int:
        return sum(1 for x in range(2, num + 1)
                   if sum([x // 1000, x % 1000 // 100, x % 100 // 10, x % 10]) % 2 == 0)
