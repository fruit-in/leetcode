# 1360. Number of Days Between Two Dates
Write a program to count the number of days between two dates.

The two dates are given as strings, their format is ```YYYY-MM-DD``` as shown in the examples.

#### Example 1:
<pre>
<strong>Input:</strong> date1 = "2019-06-29", date2 = "2019-06-30"
<strong>Output:</strong> 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> date1 = "2020-01-15", date2 = "2019-12-31"
<strong>Output:</strong> 15
</pre>

#### Constraints:
* The given dates are valid dates between the years ```1971``` and ```2100```.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let date1 = Self::from_1971_01_01(date1);
        let date2 = Self::from_1971_01_01(date2);

        (date1 - date2).abs()
    }

    pub fn from_1971_01_01(date: String) -> i32 {
        let m_d = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let date = date.split('-')
                       .map(|s| s.parse().unwrap())
                       .collect::<Vec<i32>>();

        let mut days = date[2] - 1 + m_d[date[1] as usize - 1];
        if date[0] % 4 == 0 && date[0] != 2100 && date[1] > 2 {
            days += 1;
        }
        days += 365 * (date[0] - 1971) + (date[0] - 1969) / 4;

        days
    }
}
```
