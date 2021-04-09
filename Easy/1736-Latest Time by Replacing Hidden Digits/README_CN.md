# 1736. 替换隐藏数字得到的最晚时间
给你一个字符串 `time` ，格式为 `hh:mm`（小时：分钟），其中某几位数字被隐藏（用 `?` 表示）。

有效的时间为 `00:00` 到 `23:59` 之间的所有时间，包括 `00:00` 和 `23:59` 。

替换 `time` 中隐藏的数字，返回你可以得到的最晚有效时间。

#### 示例 1:
<pre>
<strong>输入:</strong> time = "2?:?0"
<strong>输出:</strong> "23:50"
<strong>解释:</strong> 以数字 '2' 开头的最晚一小时是 23 ，以 '0' 结尾的最晚一分钟是 50 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> time = "0?:3?"
<strong>输出:</strong> "09:39"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> time = "1?:22"
<strong>输出:</strong> "19:22"
</pre>

#### 提示:
* `time` 的格式为 `hh:mm`
* 题目数据保证你可以由输入的字符串生成有效的时间

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut time = time.into_bytes();

        match (time[0], time[1]) {
            (b'?', b'?') => {
                time[0] = b'2';
                time[1] = b'3';
            }
            (b'?', t1) if t1 < b'4' => time[0] = b'2',
            (b'?', t1) => time[0] = b'1',
            (b'2', b'?') => time[1] = b'3',
            (t0, b'?') => time[1] = b'9',
            (t0, t1) => (),
        }
        if time[3] == b'?' {
            time[3] = b'5';
        }
        if time[4] == b'?' {
            time[4] = b'9';
        }

        String::from_utf8(time).unwrap()
    }
}
```
