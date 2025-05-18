class Solution:
    def minimizeSet(self, divisor1: int, divisor2: int, uniqueCnt1: int, uniqueCnt2: int) -> int:
        def isPossible(x: int) -> bool:
            divisible1, divisible2, divisible12 = x // divisor1, x // divisor2, x // divisor12
            cnt1 = max(uniqueCnt1 - divisible2 + divisible12, 0)
            cnt2 = max(uniqueCnt2 - divisible1 + divisible12, 0)

            return cnt1 + cnt2 <= x - divisible1 - divisible2 + divisible12

        divisor12 = math.lcm(divisor1, divisor2)

        return bisect.bisect(range((uniqueCnt1 + uniqueCnt2) * 3), False, key=isPossible)
