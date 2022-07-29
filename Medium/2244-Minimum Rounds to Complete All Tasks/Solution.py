class Solution:
    def minimumRounds(self, tasks: List[int]) -> int:
        count = Counter(tasks)
        ret = 0

        for v in count.values():
            if v == 1:
                return -1

            ret += v // 3 + (1 if v % 3 > 0 else 0)

        return ret
