# 2262. 字符串的总引力
字符串的 **引力** 定义为：字符串中 **不同** 字符的数量。

* 例如，`"abbca"` 的引力为 `3` ，因为其中有 `3` 个不同字符 `'a'`、`'b'` 和 `'c'` 。

给你一个字符串 `s` ，返回 **其所有子字符串的总引力** 。

**子字符串** 定义为：字符串中的一个连续字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abbca"
<strong>输出:</strong> 28
<strong>解释:</strong> "abbca" 的子字符串有：
- 长度为 1 的子字符串："a"、"b"、"b"、"c"、"a" 的引力分别为 1、1、1、1、1，总和为 5 。
- 长度为 2 的子字符串："ab"、"bb"、"bc"、"ca" 的引力分别为 2、1、2、2 ，总和为 7 。
- 长度为 3 的子字符串："abb"、"bbc"、"bca" 的引力分别为 2、2、3 ，总和为 7 。
- 长度为 4 的子字符串："abbc"、"bbca" 的引力分别为 3、3 ，总和为 6 。
- 长度为 5 的子字符串："abbca" 的引力为 3 ，总和为 3 。
引力总和为 5 + 7 + 7 + 6 + 3 = 28 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "code"
<strong>输出:</strong> 20
<strong>解释:</strong> "code" 的子字符串有：
- 长度为 1 的子字符串："c"、"o"、"d"、"e" 的引力分别为 1、1、1、1 ，总和为 4 。
- 长度为 2 的子字符串："co"、"od"、"de" 的引力分别为 2、2、2 ，总和为 6 。
- 长度为 3 的子字符串："cod"、"ode" 的引力分别为 3、3 ，总和为 6 。
- 长度为 4 的子字符串："code" 的引力为 4 ，总和为 4 。
引力总和为 4 + 6 + 6 + 4 = 20 。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn appeal_sum(s: String) -> i64 {
        let s = s.as_bytes();
        let mut last = vec![-1; 27];
        let mut mask = 0_i32;
        let mut ret = 0;

        for i in 0..s.len() {
            last[(s[i] - b'a') as usize] = i as i64;
            mask |= 1 << (s[i] - b'a');
            let mut tmp = last.clone();
            let mut appeal = mask.count_ones() as i64;
            tmp.sort_unstable();

            for j in 1..27 {
                if tmp[j] > -1 {
                    ret += (tmp[j] - tmp[j - 1]) * appeal;
                    appeal -= 1;
                }
            }
        }

        ret
    }
}
```
