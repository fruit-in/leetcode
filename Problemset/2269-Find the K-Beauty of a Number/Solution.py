class Solution:
    def divisorSubstrings(self, num: int, k: int) -> int:
        s = str(num)
        ret = 0

        for i in range(len(s) - k + 1):
            x = int(s[i:i + k])

            if x > 0 and num % x == 0:
                ret += 1

        return ret
