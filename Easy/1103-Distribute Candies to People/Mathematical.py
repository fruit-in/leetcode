class Solution:
    def distributeCandies(self, candies: int, num_people: int) -> List[int]:
        m = math.ceil(((1 + 8 * candies) ** .5 - 1) / 2)
        y = (m - 1) // num_people
        x = (m - 1) % num_people

        ret = []
        for i in range(num_people):
            if i < x:
                ret.append(y * (y + 1) * num_people // 2 + (y + 1) * (i + 1))
            elif i > x:
                ret.append(y * (y - 1) * num_people // 2 + y * (i + 1))
            else:
                ret.append(y * (y - 1) * num_people // 2 + y * (i + 1) + candies - m * (m - 1) // 2)

        return ret
