class Solution:
    def maximumSegmentSum(self, nums: List[int], removeQueries: List[int]) -> List[int]:
        from sortedcontainers import SortedList

        n = len(nums)
        prefixsum = [0] * (n + 1)
        segments = SortedList([(0, n)])
        segmentsums = SortedList([sum(nums)])
        answer = [0] * n

        for i in range(n):
            prefixsum[i + 1] = prefixsum[i] + nums[i]

        for i, j in enumerate(removeQueries[:-1]):
            k = segments.bisect_right((j, n)) - 1
            k, l = segments.pop(k)
            segmentsums.discard(prefixsum[l] - prefixsum[k])
            if j != k:
                segments.add((k, j))
                segmentsums.add(prefixsum[j] - prefixsum[k])
            if j != l - 1:
                segments.add((j + 1, l))
                segmentsums.add(prefixsum[l] - prefixsum[j + 1])

            answer[i] = segmentsums[-1]

        return answer
