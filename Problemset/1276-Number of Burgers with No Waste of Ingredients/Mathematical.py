class Solution:
    def numOfBurgers(self, tomatoSlices: int, cheeseSlices: int) -> List[int]:
        if tomatoSlices % 2 == 0 \
                and tomatoSlices >= 2 * cheeseSlices \
                and tomatoSlices <= 4 * cheeseSlices:
            return [
                tomatoSlices // 2 - cheeseSlices,
                2 * cheeseSlices - tomatoSlices // 2
            ]
        return []
