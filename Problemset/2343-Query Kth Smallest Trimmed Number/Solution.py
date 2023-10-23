class Solution:
    def smallestTrimmedNumbers(self, nums: List[str], queries: List[List[int]]) -> List[int]:
        nums = [(nums[i], i) for i in range(len(nums))]
        trims = {}
        answer = [0] * len(queries)

        for i in range(len(queries)):
            if queries[i][1] not in trims:
                trims[queries[i][1]] = []
            trims[queries[i][1]].append((i, queries[i][0] - 1))

        for trim in range(1, len(nums[0][0]) + 1):
            count = [0] * 10
            tmp = [("", 0)] * len(nums)

            for num, _ in nums:
                count[ord(num[-trim]) - 48] += 1
            for i in range(1, 10):
                count[i] += count[i - 1]
            for num, i in nums[::-1]:
                count[ord(num[-trim]) - 48] -= 1
                tmp[count[ord(num[-trim]) - 48]] = (num, i)

            for i, k in trims.get(trim, []):
                answer[i] = tmp[k][1]

            nums = tmp

        return answer
