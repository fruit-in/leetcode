class Solution:
    def numDecodings(self, s: str) -> int:
        if s[0] == '0':
            return 0

        prev, curr = 1, 1

        for i in range(1, len(s)):
            if s[i] == '0' and (s[i - 1] > '2' or s[i - 1] == '0'):
                return 0
            elif s[i] == '0':
                curr = prev
            elif "10" < s[i - 1:i + 1] < "27":
                prev, curr = curr, prev + curr
            else:
                prev = curr

        return curr
