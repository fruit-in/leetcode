class Solution:
    def simplifiedFractions(self, n: int) -> List[str]:
        return [
            "{}/{}".format(j, i)
            for i in range(2, n + 1)
            for j in range(1, i)
            if gcd(i, j) == 1
        ]
