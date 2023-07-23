class Solution:
    def numOfPairs(self, nums: List[str], target: str) -> int:
        count = {}
        ret = 0

        for num in nums:
            if target.startswith(num) and target[len(num):] in count:
                ret += count[target[len(num):]]
            if target.endswith(num) and target[:-len(num)] in count:
                ret += count[target[:-len(num)]]

            if num not in count:
                count[num] = 0
            count[num] += 1

        return ret
