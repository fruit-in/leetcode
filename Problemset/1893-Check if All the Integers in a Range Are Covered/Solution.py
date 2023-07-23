class Solution:
    def isCovered(self, ranges: List[List[int]], left: int, right: int) -> bool:
        for start, end in sorted(ranges):
            if start > left or left > right:
                break
            elif end >= left:
                left = end + 1

        return left > right
