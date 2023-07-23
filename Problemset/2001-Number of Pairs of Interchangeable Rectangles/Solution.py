class Solution:
    def interchangeableRectangles(self, rectangles: List[List[int]]) -> int:
        count = {}

        for width, height in rectangles:
            x = gcd(width, height)
            width, height = width // x, height // x
            if (width, height) not in count:
                count[(width, height)] = 0
            count[(width, height)] += 1

        return sum(x * (x - 1) // 2 for x in count.values())
