class Solution:
    def ways(self, pizza: List[str], k: int) -> int:
        @cache
        def subPizzaWays(r: int, c: int, k: int) -> int:
            if applecount[r][c] < k:
                return 0
            if k == 1:
                return 1

            ret = 0

            for i in range(r + 1, rows):
                if applecount[r][c] > applecount[i][c]:
                    ret = (ret + subPizzaWays(i, c, k - 1)) % 1000000007
            for i in range(c + 1, cols):
                if applecount[r][c] > applecount[r][i]:
                    ret = (ret + subPizzaWays(r, i, k - 1)) % 1000000007

            return ret

        rows = len(pizza)
        cols = len(pizza[0])
        applecount = [[0] * (cols) for _ in range(rows)]

        for r in range(rows - 1, -1, -1):
            for c in range(cols - 1, -1, -1):
                if pizza[r][c] == 'A':
                    applecount[r][c] = 1
                if r + 1 < rows:
                    applecount[r][c] += applecount[r + 1][c]
                if c + 1 < cols:
                    applecount[r][c] += applecount[r][c + 1]
                if r + 1 < rows and c + 1 < cols:
                    applecount[r][c] -= applecount[r + 1][c + 1]

        return subPizzaWays(0, 0, k)
