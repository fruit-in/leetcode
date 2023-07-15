# 2437. Number of Valid Clock Times
You are given a string of length `5` called `time`, representing the current time on a digital clock in the format `"hh:mm"`. The **earliest** possible time is `"00:00"` and the **latest** possible time is `"23:59"`.

In the string `time`, the digits represented by the `?` symbol are **unknown**, and must be **replaced** with a digit from `0` to `9`.

Return *an integer* `answer`, *the number of valid clock times that can be created by replacing every* `?` *with a digit from* `0` *to* `9`.

#### Example 1:
<pre>
<strong>Input:</strong> time = "?5:00"
<strong>Output:</strong> 2
<strong>Explanation:</strong> We can replace the ? with either a 0 or 1, producing "05:00" or "15:00". Note that we cannot replace it with a 2, since the time "25:00" is invalid. In total, we have two choices.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> time = "0?:0?"
<strong>Output:</strong> 100
<strong>Explanation:</strong> Each ? can be replaced by any digit from 0 to 9, so we have 100 total choices.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> time = "??:??"
<strong>Output:</strong> 1440
<strong>Explanation:</strong> There are 24 possible choices for the hours, and 60 possible choices for the minutes. In total, we have 24 * 60 = 1440 choices.
</pre>

#### Constraints:
* `time` is a valid string of length `5` in the format `"hh:mm"`.
* `"00" <= hh <= "23"`
* `"00" <= mm <= "59"`
* Some of the digits might be replaced with `'?'` and need to be replaced with digits from `0` to `9`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countTime(self, time: str) -> int:
        ret = 1

        if time[0] == '?' and time[1] == '?':
            ret *= 24
        elif time[0] == '?':
            ret *= 3 if time[1] < '4' else 2
        elif time[1] == '?':
            ret *= 10 if time[0] < '2' else 4
        ret *= 6 if time[3] == '?' else 1
        ret *= 10 if time[4] == '?' else 1

        return ret
```
