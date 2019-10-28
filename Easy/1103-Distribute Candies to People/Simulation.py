class Solution:
    def distributeCandies(self, candies: int, num_people: int) -> List[int]:
        ret = [0] * num_people
        give = 1
        i = 0

        while candies:
            ret[i] += min(give, candies)
            candies -= min(give, candies)
            give += 1
            i = (i + 1) % num_people

        return ret
