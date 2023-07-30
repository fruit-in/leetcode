class Solution:
    def intersectionSizeTwo(self, intervals: List[List[int]]) -> int:
        intervals.sort(key=lambda interval: (interval[1], interval[0]))
        nums = [intervals[0][1] - 1, intervals[0][1]]

        for start, end in intervals[1:]:
            if start <= nums[-2]:
                continue
            elif start > nums[-1]:
                nums.append(end - 1)
                nums.append(end)
            elif end > nums[-1]:
                nums.append(end)
            else:
                nums.pop()
                nums.append(end - 1)
                nums.append(end)

        return len(nums)
