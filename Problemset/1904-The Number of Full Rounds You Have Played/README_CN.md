# 1904. 你完成的完整对局数
一款新的在线电子游戏在近期发布，在该电子游戏中，以 **刻钟** 为周期规划若干时长为 **15 分钟** 的游戏对局。这意味着，在 `HH:00`、`HH:15`、`HH:30` 和 `HH:45` ，将会开始一个新的对局，其中 `HH` 用一个从 `00` 到 `23` 的整数表示。游戏中使用 **24 小时制的时钟** ，所以一天中最早的时间是 `00:00` ，最晚的时间是 `23:59` 。

给你两个字符串 `startTime` 和 `finishTime` ，均符合 `"HH:MM"` 格式，分别表示你 进入 和 退出 游戏的确切时间，请计算在整个游戏会话期间，你完成的 **完整对局的对局数** 。

* 例如，如果 `startTime = "05:20"` 且 `finishTime = "05:59"` ，这意味着你仅仅完成从 `05:30` 到 `05:45` 这一个完整对局。而你没有完成从 `05:15` 到 `05:30` 的完整对局，因为你是在对局开始后进入的游戏；同时，你也没有完成从 `05:45` 到 `06:00` 的完整对局，因为你是在对局结束前退出的游戏。

如果 `finishTime` **早于** `startTime` ，这表示你玩了个通宵（也就是从 `startTime` 到午夜，再从午夜到 `finishTime`）。

假设你是从 `startTime` 进入游戏，并在 `finishTime` 退出游戏，请计算并返回你完成的 **完整对局的对局数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> startTime = "12:01", finishTime = "12:44"
<strong>输出:</strong> 1
<strong>解释:</strong> 你完成了从 12:15 到 12:30 的一个完整对局。
你没有完成从 12:00 到 12:15 的完整对局，因为你是在对局开始后的 12:01 进入的游戏。
你没有完成从 12:30 到 12:45 的完整对局，因为你是在对局结束前的 12:44 退出的游戏。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> startTime = "20:00", finishTime = "06:00"
<strong>输出:</strong> 40
<strong>解释:</strong> 你完成了从 20:00 到 00:00 的 16 个完整的对局，以及从 00:00 到 06:00 的 24 个完整的对局。
16 + 24 = 40
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> startTime = "00:00", finishTime = "23:59"
<strong>输出:</strong> 95
<strong>解释:</strong> 除最后一个小时你只完成了 3 个完整对局外，其余每个小时均完成了 4 场完整对局。
</pre>

#### 提示:
* `startTime` 和 `finishTime` 的格式为 `HH:MM`
* `00 <= HH <= 23`
* `00 <= MM <= 59`
* `startTime` 和 `finishTime` 不相等

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def numberOfRounds(self, loginTime: str, logoutTime: str) -> int:
        loginhour = int(loginTime[:2])
        loginmin = int(loginTime[3:])
        logouthour = int(logoutTime[:2]) + \
            (24 if logoutTime < loginTime else 0)
        logoutmin = int(logoutTime[3:])
        ret = (logouthour + 1 - loginhour) * 4

        if loginmin > 45:
            ret -= 4
        elif loginmin > 30:
            ret -= 3
        elif loginmin > 15:
            ret -= 2
        elif loginmin > 0:
            ret -= 1
        if logoutmin < 15:
            ret -= 4
        elif logoutmin < 30:
            ret -= 3
        elif logoutmin < 45:
            ret -= 2
        else:
            ret -= 1

        return max(ret, 0)
```
