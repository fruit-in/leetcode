class Solution:
    def removeInvalidParentheses(self, s: str) -> List[str]:
        used = [False] * len(s)
        maxlength = 0
        ret = {""}

        for i in range(len(s)):
            if s[i] not in "()":
                used[i] = True

        def dfs(i: int, opening: int, length: int) -> None:
            nonlocal maxlength, ret
            if i == len(s):
                if opening == 0:
                    if length > maxlength:
                        maxlength = length
                        ret = set()
                    if length == maxlength:
                        ret.add(''.join(s[i]
                                for i in range(len(s)) if used[i]))
                return
            if len(s) - i < opening or length + len(s) - i < maxlength:
                return

            if s[i] == '(':
                used[i] = True
                dfs(i + 1, opening + 1, length + 1)
                used[i] = False
                dfs(i + 1, opening, length)
            elif s[i] == ')':
                if opening > 0:
                    used[i] = True
                    dfs(i + 1, opening - 1, length + 1)
                    used[i] = False
                dfs(i + 1, opening, length)
            else:
                dfs(i + 1, opening, length + 1)

        dfs(0, 0, 0)

        return list(ret)
