# 1190. 反转每对括号间的子串
给出一个字符串 `s`（仅含有小写英文字母和括号）。

请你按照从括号内到外的顺序，逐层反转每对匹配括号中的字符串，并返回最终的结果。

注意，您的结果中 **不应** 包含任何括号。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "(abcd)"
<strong>输出:</strong> "dcba"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "(u(love)i)"
<strong>输出:</strong> "iloveu"
<strong>解释:</strong> 先反转子字符串 "love" ，然后反转整个字符串。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "(ed(et(oc))el)"
<strong>输出:</strong> "leetcode"
<strong>解释:</strong> 先反转子字符串 "oc" ，接着反转 "etco" ，然后反转整个字符串。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "a(bcdefghijkl(mno)p)q"
<strong>输出:</strong> "apmnolkjihgfedcbq"
</pre>

#### 提示:
* `1 <= s.length <= 2000`
* `s` 中只有小写英文字母和括号
* 题目测试用例确保所有括号都是成对出现的

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut ret = vec![];

        for ch0 in s.bytes() {
            if ch0 == b')' {
                let mut stack = vec![];

                while let Some(ch1) = ret.pop() {
                    if ch1 == b'(' {
                        break;
                    }
                    stack.push(ch1);
                }

                ret.append(&mut stack);
            } else {
                ret.push(ch0);
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
```
