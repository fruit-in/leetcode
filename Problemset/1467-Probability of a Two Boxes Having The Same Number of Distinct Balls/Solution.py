from math import comb


class Solution:
    def getProbability(self, balls: List[int]) -> float:
        n = sum(balls) // 2
        k = len(balls)
        used = 0
        dp = [[[0] * (k * 2 + 1) for _ in range(n * 2 + 1)]
              for _ in range(n * 2 + 1)]
        dp[0][0][k] = 1

        for i in range(k):
            tmp = [[[0] * (k * 2 + 1) for _ in range(n * 2 + 1)]
                   for _ in range(n * 2 + 1)]

            for leftballs, rightballs in zip(range(used + 1), range(used, -1, -1)):
                for offsetdiff in range(len(dp[0][0])):
                    if rightballs + balls[i] < len(dp) and offsetdiff > 0:
                        tmp[leftballs][rightballs + balls[i]][offsetdiff -
                                                              1] += dp[leftballs][rightballs][offsetdiff] * comb(rightballs + balls[i], balls[i])
                    for toleft, toright in zip(range(1, balls[i]), range(balls[i] - 1, 0, -1)):
                        if leftballs + toleft < len(dp) and rightballs + toright < len(dp):
                            tmp[leftballs + toleft][rightballs + toright][offsetdiff] += dp[leftballs][rightballs][offsetdiff] * comb(
                                leftballs + toleft, toleft) * comb(rightballs + toright, toright)
                    if leftballs + balls[i] < len(dp) and offsetdiff + 1 < len(dp[0][0]):
                        tmp[leftballs + balls[i]][rightballs][offsetdiff +
                                                              1] += dp[leftballs][rightballs][offsetdiff] * comb(leftballs + balls[i], balls[i])

            used += balls[i]
            dp = tmp

        return dp[n][n][k] / sum(dp[n][n])
