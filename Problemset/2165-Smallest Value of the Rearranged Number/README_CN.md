# 2165. 重排数字的最小值
给你一个整数 `num` 。**重排** `num` 中的各位数字，使其值 **最小化** 且不含 **任何** 前导零。

返回不含前导零且值最小的重排数字。

注意，重排各位数字后，`num` 的符号不会改变。

#### 示例 1:
<pre>
<strong>输入:</strong> num = 310
<strong>输出:</strong> 103
<strong>解释:</strong> 310 中各位数字的可行排列有：013、031、103、130、301、310 。
不含任何前导零且值最小的重排数字是 103 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = -7605
<strong>输出:</strong> -7650
<strong>解释:</strong> -7605 中各位数字的部分可行排列为：-7650、-6705、-5076、-0567。
不含任何前导零且值最小的重排数字是 -7650 。
</pre>

#### 提示:
* <code>-10<sup>15</sup> <= num <= 10<sup>15</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn smallest_number(num: i64) -> i64 {
        let mut digits = num.abs().to_string().into_bytes();

        digits.sort_unstable();
        if num >= 0 {
            for i in 0..digits.len() {
                if digits[i] != b'0' {
                    digits.swap(0, i);
                    break;
                }
            }
        } else {
            digits.push(b'-');
            digits.reverse();
        }

        String::from_utf8(digits).unwrap().parse().unwrap()
    }
}
```
