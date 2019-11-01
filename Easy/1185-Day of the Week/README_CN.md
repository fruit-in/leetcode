# 1185. 一周中的第几天
给你一个日期，请你设计一个算法来判断它是对应一周中的哪一天。

输入为三个整数：```day```、```month``` 和 ```year```，分别表示日、月、年。

您返回的结果必须是这几个值中的一个 ```{"Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"}```。

#### 示例 1:
<pre>
<strong>输入:</strong> day = 31, month = 8, year = 2019
<strong>输出:</strong> "Saturday"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> day = 18, month = 7, year = 1999
<strong>输出:</strong> "Sunday"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> day = 15, month = 8, year = 1993
<strong>输出:</strong> "Sunday"
</pre>

#### 提示:
* 给出的日期一定是在 ```1971``` 到 ```2100``` 年之间的有效日期。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let mut day = day;
        let mut months = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let week = ["Thursday", "Friday", "Saturday",
            "Sunday", "Monday", "Tuesday", "Wednesday"];
 
        day += (year - 1971) * 365 + (year - 1969) / 4 + months[month as usize - 1];
        if year % 4 == 0 && month > 2 {
            day += 1;
        }
        week[day as usize % 7].to_string()
    }
}
```
