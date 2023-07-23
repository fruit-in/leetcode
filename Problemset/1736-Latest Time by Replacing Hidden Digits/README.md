# 1736. Latest Time by Replacing Hidden Digits
You are given a string `time` in the form of `hh:mm`, where some of the digits in the string are hidden (represented by `?`).

The valid times are those inclusively between `00:00` and `23:59`.

Return *the latest valid time you can get from* `time` *by replacing the hidden digits*.

#### Example 1:
<pre>
<strong>Input:</strong> time = "2?:?0"
<strong>Output:</strong> "23:50"
<strong>Explanation:</strong> The latest hour beginning with the digit '2' is 23 and the latest minute ending with the digit '0' is 50.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> time = "0?:3?"
<strong>Output:</strong> "09:39"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> time = "1?:22"
<strong>Output:</strong> "19:22"
</pre>

#### Constraints:
* `time` is in the format `hh:mm`.
* It is guaranteed that you can produce a valid time from the given string.

## Solutions (Rust)

### 1. Solution
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
