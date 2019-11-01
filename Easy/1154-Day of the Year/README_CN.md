# 1154. 一年中的第几天
给你一个按 ```YYYY-MM-DD``` 格式表示日期的字符串 ```date```，请你计算并返回该日期是当年的第几天。

通常情况下，我们认为 1 月 1 日是每年的第 1 天，1 月 2 日是每年的第 2 天，依此类推。每个月的天数与现行公元纪年法（格里高利历）一致。

#### 示例 1:
<pre>
<strong>输入:</strong> date = "2019-01-09"
<strong>输出:</strong> 9
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> date = "2019-02-10"
<strong>输出:</strong> 41
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> date = "2003-03-01"
<strong>输出:</strong> 60
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> date = "2004-03-01"
<strong>输出:</strong> 61
</pre>

#### 提示:
* ```date.length == 10```
* ```date[4] == date[7] == '-'```，其他的 ```date[i]``` 都是数字。
* ```date``` 表示的范围从 1900 年 1 月 1 日至 2019 年 12 月 31 日。

## 题解 (Rust)

### 1. 题解
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
