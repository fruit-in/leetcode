class Solution:
    def kIncreasing(self, arr: List[int], k: int) -> int:
        groups = [[] for _ in range(k)]
        ret = 0

        for i in range(len(arr)):
            groups[i % k].append(arr[i])

        for group in groups:
            lis = [0]

            for x in group:
                i = bisect.bisect(lis, x)
                if i == len(lis):
                    lis.append(x)
                else:
                    lis[i] = x

            ret += len(group) - len(lis) + 1

        return ret
