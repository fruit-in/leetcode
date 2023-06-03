# 1759. 统计同质子字符串的数目
给你一个字符串 `s` ，返回 `s` 中 **同质子字符串** 的数目。由于答案可能很大，只需返回对 <code>10<sup>9</sup> + 7</code> **取余** 后的结果。

**同质字符串** 的定义为：如果一个字符串中的所有字符都相同，那么该字符串就是同质字符串。

**子字符串** 是字符串中的一个连续字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abbcccaa"
<strong>输出:</strong> 13
<strong>解释:</strong> 同质子字符串如下所列：
"a"   出现 3 次。
"aa"  出现 1 次。
"b"   出现 2 次。
"bb"  出现 1 次。
"c"   出现 3 次。
"cc"  出现 2 次。
"ccc" 出现 1 次。
3 + 1 + 2 + 1 + 3 + 2 + 1 = 13
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "xy"
<strong>输出:</strong> 2
<strong>解释:</strong> 同质子字符串是 "x" 和 "y" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "zzzzz"
<strong>输出:</strong> 15
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 由小写字符串组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = 1;
        let mut ret = 1;

        for i in 1..s.len() {
            if s[i] != s[i - 1] {
                count = 0;
            }

            count += 1;
            ret = (ret + count) % 1_000_000_007;
        }

        ret
    }
}
```
