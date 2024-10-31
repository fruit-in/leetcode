# 2301. 替换字符后匹配
给你两个字符串 `s` 和 `sub` 。同时给你一个二维字符数组 `mappings` ，其中 <code>mappings[i] = [old<sub>i</sub>, new<sub>i</sub>]</code> 表示你可以将 `sub` 中任意数目的 <code>old<sub>i</sub></code> 字符替换为 <code>new<sub>i</sub></code> 。`sub` 中每个字符 **不能** 被替换超过一次。

如果使用 `mappings` 替换 0 个或者若干个字符，可以将 `sub` 变成 `s` 的一个子字符串，请你返回 `true`，否则返回 `false` 。

一个 **子字符串** 是字符串中连续非空的字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "fool3e7bar", sub = "leet", mappings = [["e","3"],["t","7"],["t","8"]]
<strong>输出:</strong> true
<strong>解释:</strong> 将 sub 中第一个 'e' 用 '3' 替换，将 't' 用 '7' 替换。
现在 sub = "l3e7" ，它是 s 的子字符串，所以我们返回 true 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "fooleetbar", sub = "f00l", mappings = [["o","0"]]
<strong>输出:</strong> false
<strong>解释:</strong> 字符串 "f00l" 不是 s 的子串且没有可以进行的修改。
注意我们不能用 'o' 替换 '0' 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "Fool33tbaR", sub = "leetd", mappings = [["e","3"],["t","7"],["t","8"],["d","b"],["p","b"]]
<strong>输出:</strong> true
<strong>解释:</strong> 将 sub 里第一个和第二个 'e' 用 '3' 替换，用 'b' 替换 sub 里的 'd' 。
得到 sub = "l33tb" ，它是 s 的子字符串，所以我们返回 true 。
</pre>

#### 提示:
* `1 <= sub.length <= s.length <= 5000`
* `0 <= mappings.length <= 1000`
* `mappings[i].length == 2`
* <code>old<sub>i</sub> != new<sub>i</sub></code>
* `s` 和 `sub` 只包含大写和小写英文字母和数字。
* <code>old<sub>i</sub></code> 和 <code>new<sub>i</sub></code> 是大写、小写字母或者是个数字。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
        let s = s.as_bytes();
        let sub = sub.as_bytes();
        let mut bitmappings = [0_i128; 128];

        for i in 0..mappings.len() {
            let (old, new) = (mappings[i][0] as usize, mappings[i][1] as usize);
            bitmappings[old] |= 1 << new;
        }

        for i in 0..=s.len() - sub.len() {
            let mut flag = true;

            for j in 0..sub.len() {
                if sub[j] != s[i + j] && bitmappings[sub[j] as usize] & (1 << s[i + j]) == 0 {
                    flag = false;
                    break;
                }
            }

            if flag {
                return true;
            }
        }

        false
    }
}
```
