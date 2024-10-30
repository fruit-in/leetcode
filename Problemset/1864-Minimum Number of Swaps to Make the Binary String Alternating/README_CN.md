# 1864. 构成交替字符串需要的最小交换次数
给你一个二进制字符串 `s` ，现需要将其转化为一个 **交替字符串** 。请你计算并返回转化所需的 **最小** 字符交换次数，如果无法完成转化，返回 `-1` 。

**交替字符串** 是指：相邻字符之间不存在相等情况的字符串。例如，字符串 `"010"` 和 `"1010"` 属于交替字符串，但 `"0100"` 不是。

任意两个字符都可以进行交换，**不必相邻** 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "111000"
<strong>输出:</strong> 1
<strong>解释:</strong> 交换位置 1 和 4："111000" -> "101010" ，字符串变为交替字符串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "010"
<strong>输出:</strong> 0
<strong>解释:</strong> 字符串已经是交替字符串了，不需要交换。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "1110"
<strong>输出:</strong> -1
</pre>

#### 提示:
* `1 <= s.length <= 1000`
* `s[i]` 的值为 `'0'` 或 `'1'`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let zeros = s.chars().filter(|&c| c == '0').count();
        let ones = s.len() - zeros;
        let mut ret0 = i32::MAX;
        let mut ret1 = i32::MAX;

        if zeros == ones || zeros == ones + 1 {
            ret0 = s
                .bytes()
                .enumerate()
                .filter(|&(i, c)| (i % 2) as u8 + b'0' != c)
                .count() as i32
                / 2;
        }
        if zeros == ones || zeros + 1 == ones {
            ret1 = s
                .bytes()
                .enumerate()
                .filter(|&(i, c)| (i % 2) as u8 + b'0' == c)
                .count() as i32
                / 2;
        }

        if ret0 == i32::MAX && ret1 == i32::MAX {
            return -1;
        }

        ret0.min(ret1)
    }
}
```
