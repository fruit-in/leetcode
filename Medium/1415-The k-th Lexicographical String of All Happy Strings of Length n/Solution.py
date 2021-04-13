class Solution:
    def getHappyString(self, n: int, k: int) -> str:
        happy = list("abc")

        for _ in range(n - 1):
            happy_ = []

            for s in happy:
                for c in "abc":
                    if s[-1] != c:
                        happy_.append(s + c)

            happy = happy_

        return "" if k > len(happy) else happy[k - 1]
