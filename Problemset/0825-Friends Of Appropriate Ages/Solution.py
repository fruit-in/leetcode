class Solution:
    def numFriendRequests(self, ages: List[int]) -> int:
        n = len(ages)
        ret = 0

        ages.sort()

        for age in ages:
            if age > 14:
                ret += n - 1
                ret -= bisect.bisect_right(ages, 0.5 * age + 7)
                ret -= n - bisect.bisect_right(ages, age)

        return ret
