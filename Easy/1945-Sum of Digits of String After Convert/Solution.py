class Solution:
    def getLucky(self, s: str, k: int) -> int:
        ret = int(''.join(str(ord(c) - ord('a') + 1) for c in s))

        for _ in range(k):
            ret = sum(int(x) for x in str(ret))

        return ret
