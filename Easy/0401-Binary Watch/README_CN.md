# 401. 二进制手表
二进制手表顶部有 4 个 LED 代表**小时（0-11）**，底部的 6 个 LED 代表**分钟（0-59）**。

每个 LED 代表一个 0 或 1，最低位在右侧。

![](https://upload.wikimedia.org/wikipedia/commons/8/8b/Binary_clock_samui_moon.jpg)

例如，上面的二进制手表读取 “3:25”。

给定一个非负整数 *n* 代表当前 LED 亮着的数量，返回所有可能的时间。

#### 案例:
```
输入: n = 1
返回: ["1:00", "2:00", "4:00", "8:00", "0:01", "0:02", "0:04", "0:08", "0:16", "0:32"]
```

#### 注意事项:
* 输出的顺序没有要求。
* 小时不会以零开头，比如 “01:00” 是不允许的，应为 “1:00”。
* 分钟必须由两位数组成，可能会以零开头，比如 “10:2” 是无效的，应为 “10:02”。

## 题解 (Python)

### 1. 暴力法
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

### 2. 回溯
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
