class Solution:
    def distributeCookies(self, cookies: List[int], k: int) -> int:
        def dfs(i: int) -> None:
            nonlocal ret
            if i == len(cookies):
                ret = min(ret, max(children))
                return

            for j in range(k):
                if children[j] + cookies[i] < ret and (j == 0 or children[j] != children[j - 1]):
                    children[j] += cookies[i]
                    dfs(i + 1)
                    children[j] -= cookies[i]

        children = [0] * k
        children[0] = cookies[0]
        ret = max(max(cookies[:k - 1]), sum(cookies[k - 1:]))

        dfs(1)

        return ret
