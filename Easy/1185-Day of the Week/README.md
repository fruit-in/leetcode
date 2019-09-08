# 1185. Day of the Week
Given a date, return the corresponding day of the week for that date.

The input is given as three integers representing the ```day```, ```month``` and ```year``` respectively.

Return the answer as one of the following values ```{"Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"}```.

#### Example 1:
<pre>
<strong>Input:</strong> day = 31, month = 8, year = 2019
<strong>Output:</strong> "Saturday"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> day = 18, month = 7, year = 1999
<strong>Output:</strong> "Sunday"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> day = 15, month = 8, year = 1993
<strong>Output:</strong> "Sunday"
</pre>

#### Constraints:
* The given dates are valid dates between the years ```1971``` and ```2100```.

## Solutions (Rust)

### 1. Solution
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
