from functools import cache


class Solution:
    squares = [1]

    @cache
    def winnerSquareGame(self, n: int) -> bool:
        while n > self.squares[-1]:
            self.squares.append((len(self.squares) + 1) ** 2)

        return any(not self.winnerSquareGame(n - x) for x in self.squares[::-1] if x <= n)
