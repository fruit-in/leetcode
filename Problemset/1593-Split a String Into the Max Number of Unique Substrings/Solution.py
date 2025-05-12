class Solution:
    def maxUniqueSplit(self, s: str) -> int:
        ret = 1

        def dfs(i: int, subs: Set[str]) -> None:
            nonlocal ret
            if i == len(s):
                ret = max(ret, len(subs))
            if len(subs) + len(s) - i <= ret:
                return

            for j in range(i, len(s)):
                if s[i:j + 1] not in subs:
                    dfs(j + 1, subs | {s[i:j + 1]})

        dfs(0, set())

        return ret
