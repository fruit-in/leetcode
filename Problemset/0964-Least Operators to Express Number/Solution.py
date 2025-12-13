class Solution:
    def leastOpsExpressTarget(self, x: int, target: int) -> int:
        @cache
        def dfs(target: int) -> int:
            if target < x:
                return min(target * 2 - 1, (x - target) * 2)

            exp = int(log(target, x))
            val = x ** exp

            if val == target:
                return exp - 1

            ret = dfs(target - val) + exp
            if val * x - target < target:
                ret = min(ret, dfs(val * x - target) + exp + 1)

            return ret

        return dfs(target)
