# 2315. 统计星号
给你一个字符串 `s` ，每 **两个** 连续竖线 `'|'` 为 一对 。换言之，第一个和第二个 `'|'` 为一对，第三个和第四个 `'|'` 为一对，以此类推。

请你返回 **不在** 竖线对之间，`s` 中 `'*'` 的数目。

**注意**，每个竖线 `'|'` 都会 **恰好** 属于一个对。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "l|*e*et|c**o|*de|"
<strong>输出:</strong> 2
<strong>解释:</strong> 不在竖线对之间的字符加粗加斜体后，得到字符串："l|*e*et|c**o|*de|" 。
第一和第二条竖线 '|' 之间的字符不计入答案。
同时，第三条和第四条竖线 '|' 之间的字符也不计入答案。
不在竖线对之间总共有 2 个星号，所以我们返回 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "iamprogrammer"
<strong>输出:</strong> 0
<strong>解释:</strong> 在这个例子中，s 中没有星号。所以返回 0 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "yo|uar|e**|b|e***au|tifu|l"
<strong>输出:</strong> 5
<strong>解释:</strong> 需要考虑的字符加粗加斜体后："yo|uar|e**|b|e***au|tifu|l" 。不在竖线对之间总共有 5 个星号。所以我们返回 5 。
</pre>

#### 提示:
* `1 <= s.length <= 1000`
* `s` 只包含小写英文字母，竖线 `'|'` 和星号 `'*'` 。
* `s` 包含 **偶数** 个竖线 `'|'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut in_pair = false;
        let mut ret = 0;

        for c in s.chars() {
            if !in_pair && c == '*' {
                ret += 1;
            } else if c == '|' {
                in_pair = !in_pair;
            }
        }

        ret
    }
}
```
