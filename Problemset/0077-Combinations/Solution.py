class Solution:
    def combine(self, n: int, k: int) -> List[List[int]]:
        def foo(s: int, n: int, k: int) -> List[List[int]]:
            if k == 1:
                return [[i] for i in range(s, n + 1)]

            ret = []

            for i in range(s, n - k + 2):
                combs = foo(i + 1, n, k - 1)
                for comb in combs:
                    comb.append(i)
                ret.extend(combs)

            return ret

        return foo(1, n, k)
