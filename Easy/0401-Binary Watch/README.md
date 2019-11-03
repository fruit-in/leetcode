# 401. Binary Watch
A binary watch has 4 LEDs on the top which represent the **hours** (**0-11**), and the 6 LEDs on the bottom represent the **minutes** (**0-59**).

Each LED represents a zero or one, with the least significant bit on the right.

![](https://upload.wikimedia.org/wikipedia/commons/8/8b/Binary_clock_samui_moon.jpg)

For example, the above binary watch reads "3:25".

Given a non-negative integer *n* which represents the number of LEDs that are currently on, return all possible times the watch could represent.

#### Example:
```
Input: n = 1
Return: ["1:00", "2:00", "4:00", "8:00", "0:01", "0:02", "0:04", "0:08", "0:16", "0:32"]
```

#### Note:
* The order of output does not matter.
* The hour must not contain a leading zero, for example "01:00" is not valid, it should be "1:00".
* The minute must be consist of two digits and may contain a leading zero, for example "10:2" is not valid, it should be "10:02".

## Solutions (Python)

### 1. Brute Force
```Python3
class Solution:
    def readBinaryWatch(self, num: int) -> List[str]:
        hours = [[], [], [], []]
        mins = [[], [], [], [], [], []]
        time = []

        for n in range(0, 60):
            if n < 12:
                hours[bin(n).count('1')].append(n)
            mins[bin(n).count('1')].append(n)

        for i in range(0, num + 1):
            if i < 4 and num - i < 6:
                time.extend("%d:%02d" % (h, m) for h in hours[i] for m in mins[num - i])

        return time
```

### 2. Backtracking
```Python3
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
```
