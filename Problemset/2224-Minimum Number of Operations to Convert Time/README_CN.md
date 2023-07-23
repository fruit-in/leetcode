# 2224. 转化时间需要的最少操作数
给你两个字符串 `current` 和 `correct` ，表示两个 **24 小时制时间** 。

**24 小时制时间** 按 `"HH:MM"` 进行格式化，其中 `HH` 在 `00` 和 `23` 之间，而 `MM` 在 `00` 和 `59` 之间。最早的 24 小时制时间为 `00:00` ，最晚的是 `23:59` 。

在一步操作中，你可以将 `current` 这个时间增加 `1`、`5`、`15` 或 `60` 分钟。你可以执行这一操作 **任意** 次数。

返回将 `current` 转化为 `correct` 需要的 **最少操作数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> current = "02:30", correct = "04:35"
<strong>输出:</strong> 3
<strong>解释:</strong>
可以按下述 3 步操作将 current 转换为 correct ：
- 为 current 加 60 分钟，current 变为 "03:30" 。
- 为 current 加 60 分钟，current 变为 "04:30" 。
- 为 current 加 5 分钟，current 变为 "04:35" 。
可以证明，无法用少于 3 步操作将 current 转化为 correct 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> current = "11:00", correct = "11:01"
<strong>输出:</strong> 1
<strong>解释:</strong> 只需要为 current 加一分钟，所以最小操作数是 1 。
</pre>

#### 提示:
* `current` 和 `correct` 都符合 `"HH:MM"` 格式
* `current <= correct`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def convertTime(self, current: str, correct: str) -> int:
        current = int(current[:2]) * 60 + int(current[3:])
        correct = int(correct[:2]) * 60 + int(correct[3:])
        diff = correct - current + (1440 if correct < current else 0)
        ret = 0

        ret, diff = ret + diff // 60,  diff % 60
        ret, diff = ret + diff // 15,  diff % 15
        ret, diff = ret + diff // 5,  diff % 5
        ret += diff

        return ret
```
