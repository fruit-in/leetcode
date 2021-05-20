# 1796. 字符串中第二大的数字
给你一个混合字符串 `s` ，请你返回 `s` 中 **第二大** 的数字，如果不存在第二大的数字，请你返回 `-1` 。

**混合字符串** 由小写英文字母和数字组成。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "dfa12321afd"
<strong>输出:</strong> 2
<strong>解释:</strong> 出现在 s 中的数字包括 [1, 2, 3] 。第二大的数字是 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abc1111"
<strong>输出:</strong> -1
<strong>解释:</strong> 出现在 s 中的数字只包含 [1] 。没有第二大的数字。
</pre>

#### 提示:
* `1 <= s.length <= 500`
* `s` 只包含小写英文字母和（或）数字。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut digits = [false; 10];

        for c in s.bytes() {
            if c.is_ascii_digit() {
                digits[(c - b'0') as usize] = true;
            }
        }

        (0..=9)
            .rev()
            .skip_while(|&x| !digits[x as usize])
            .skip(1)
            .find(|&x| digits[x as usize])
            .unwrap_or(-1)
    }
}
```
