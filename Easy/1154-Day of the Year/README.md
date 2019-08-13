# 1154. Day of the Year
Given a string <code>date</code> representing a [Gregorian calendar](https://en.wikipedia.org/wiki/Gregorian_calendar) date formatted as <code>YYYY-MM-DD</code>, return the day number of the year.

#### Example 1:
<pre>
<strong>Input:</strong> date = "2019-01-09"
<strong>Output:</strong> 9
<strong>Explanation:</strong> Given date is the 9th day of the year in 2019.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> date = "2019-02-10"
<strong>Output:</strong> 41
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> date = "2003-03-01"
<strong>Output:</strong> 60
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> date = "2004-03-01"
<strong>Output:</strong> 61
</pre>

#### Constraints:
* <code>date.length == 10</code>
* <code>date[4] == date[7] == '-'</code>, and all other <code>date[i]</code>'s are digits
* <code>date</code> represents a calendar date between Jan 1st, 1900 and Dec 31, 2019.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let v: Vec<i32> = date.split('-').map(|s| s.parse().unwrap()).collect();
        let mut m = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30];
        let mut ans = v[2];
        if (v[0] % 4 == 0 && v[0] % 100 != 0) || v[0] % 400 == 0 {
            m[1] += 1;
        }
        for i in 1..v[1] {
            ans += m[i as usize - 1];
        }
        ans
    }
}
```
