# 1360. 日期之间隔几天
请你编写一个程序来计算两个日期之间隔了多少天。

日期以字符串形式给出，格式为 ```YYYY-MM-DD```，如示例所示。

#### 示例 1:
<pre>
<strong>输入:</strong> date1 = "2019-06-29", date2 = "2019-06-30"
<strong>输出:</strong> 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> date1 = "2020-01-15", date2 = "2019-12-31"
<strong>输出:</strong> 15
</pre>

#### 提示:
* 给定的日期是 ```1971``` 年到 ```2100``` 年之间的有效日期。

## 题解 (Rust)

### 1. 题解
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
