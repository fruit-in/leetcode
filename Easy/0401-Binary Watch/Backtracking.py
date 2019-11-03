class Solution:
    def readBinaryWatch(self, num: int) -> List[str]:
        def helper(num: int, leds: List[int]) -> List[int]:
            if num == 0:
                return [0]

            time = []

            for i in range(0, len(leds) - num + 1):
                time.extend(leds[i] + t for t in helper(num - 1, leds[i + 1:]))

            return time


        ret = []

        for i in range(0, num + 1):
            if i < 4 and num - i < 6:
                hours = helper(i, [1, 2, 4, 8])
                minutes = helper(num - i, [1, 2, 4, 8, 16, 32])

                for hour in hours:
                    for minute in minutes:
                        if hour < 12 and minute < 60:
                            ret.append("%d:%02d" % (hour, minute))

        return ret
