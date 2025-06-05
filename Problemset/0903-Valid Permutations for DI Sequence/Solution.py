class Solution:
    def numPermsDISequence(self, s: str) -> int:
        @cache
        def dfs(i: int, greater: int) -> int:
            if i == len(s):
                return 1

            less = len(s) - i - greater
            ret = 0

            if s[i] == 'D':
                for x in range(less):
                    ret = (ret + dfs(i + 1, less - 1 - x + greater)) % 1000000007
            else:
                for x in range(greater):
                    ret = (ret + dfs(i + 1, greater - 1 - x)) % 1000000007

            return ret

        s = "I" + s

        return dfs(0, len(s))
