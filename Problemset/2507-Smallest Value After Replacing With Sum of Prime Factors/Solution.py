class Solution:
    lpf = [0] * 100001
    for i in range(2, 100001):
        if lpf[i] == 0:
            for j in range(i, 100001, i):
                if lpf[j] == 0:
                    lpf[j] = i

    def smallestValue(self, n: int) -> int:
        while self.lpf[n] != n:
            x = n
            pfsum = 0

            while x > 1:
                pfsum += self.lpf[x]
                x //= self.lpf[x]

            if n == pfsum:
                break

            n = pfsum

        return n
