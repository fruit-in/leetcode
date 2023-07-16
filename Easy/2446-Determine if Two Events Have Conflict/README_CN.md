# 2446. 判断两个事件是否存在冲突
给你两个字符串数组 `event1` 和 `event2` ，表示发生在同一天的两个闭区间时间段事件，其中：

* <code>event1 = [startTime<sub>1</sub>, endTime<sub>1</sub>]</code> 且
* <code>event2 = [startTime<sub>2</sub>, endTime<sub>2</sub>]</code>

事件的时间为有效的 24 小时制且按 `HH:MM` 格式给出。

当两个事件存在某个非空的交集时（即，某些时刻是两个事件都包含的），则认为出现 **冲突** 。

如果两个事件之间存在冲突，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> event1 = ["01:15","02:00"], event2 = ["02:00","03:00"]
<strong>输出:</strong> true
<strong>解释:</strong> 两个事件在 2:00 出现交集。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> event1 = ["01:00","02:00"], event2 = ["01:20","03:00"]
<strong>输出:</strong> true
<strong>解释:</strong> 两个事件的交集从 01:20 开始，到 02:00 结束。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> event1 = ["10:00","11:00"], event2 = ["14:00","15:00"]
<strong>输出:</strong> false
<strong>解释:</strong> 两个事件不存在交集。
</pre>

#### 提示:
* `evnet1.length == event2.length == 2.`
* `event1[i].length == event2[i].length == 5`
* <code>startTime<sub>1</sub> <= endTime<sub>1</sub></code>
* <code>startTime<sub>2</sub> <= endTime<sub>2</sub></code>
* 所有事件的时间都按照 `HH:MM` 格式给出

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def haveConflict(self, event1: List[str], event2: List[str]) -> bool:
        start1 = int(event1[0][:2]) * 60 + int(event1[0][3:])
        end1 = int(event1[1][:2]) * 60 + int(event1[1][3:])
        start2 = int(event2[0][:2]) * 60 + int(event2[0][3:])
        end2 = int(event2[1][:2]) * 60 + int(event2[1][3:])

        return (end1 >= start2 and end1 <= end2) or (end2 >= start1 and end2 <= end1)
```
