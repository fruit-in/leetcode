# 1507. 转变日期格式
给你一个字符串 `date` ，它的格式为 `Day Month Year` ，其中：
* `Day` 是集合 `{"1st", "2nd", "3rd", "4th", ..., "30th", "31st"}` 中的一个元素。
* `Month` 是集合 `{"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"}` 中的一个元素。
* `Year` 的范围在 `[1900, 2100]` 之间。

请你将字符串转变为 `YYYY-MM-DD` 的格式，其中：
* `YYYY` 表示 4 位的年份。
* `MM` 表示 2 位的月份。
* `DD` 表示 2 位的天数。

#### 示例 1:
<pre>
<strong>输入:</strong> date = "20th Oct 2052"
<strong>输出:</strong> "2052-10-20"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> date = "6th Jun 1933"
<strong>输出:</strong> "1933-06-06"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> date = "26th May 1960"
<strong>输出:</strong> "1960-05-26"
</pre>

#### 提示:
* 给定日期保证是合法的，所以不需要处理异常输入。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def reformatDate(self, date: str) -> str:
        day, month, year = date.split()
        day = int(day[:-2])
        month = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"].index(month) + 1

        return "%s-%02d-%02d" % (year, month, day)
```
