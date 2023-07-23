# 2375. 根据模式串构造最小数字
给你下标从 **0** 开始、长度为 `n` 的字符串 `pattern` ，它包含两种字符，`'I'` 表示 **上升** ，`'D'` 表示 **下降** 。

你需要构造一个下标从 **0** 开始长度为 `n + 1` 的字符串，且它要满足以下条件：

* `num` 包含数字 `'1'` 到 `'9'` ，其中每个数字 **至多** 使用一次。
* 如果 `pattern[i] == 'I'` ，那么 `num[i] < num[i + 1]` 。
* 如果 `pattern[i] == 'D'` ，那么 `num[i] > num[i + 1]` 。

请你返回满足上述条件字典序 **最小** 的字符串 `num`。

#### 示例 1:
<pre>
<strong>输入:</strong> pattern = "IIIDIDDD"
<strong>输出:</strong> "123549876"
<strong>解释:</strong>
下标 0 ，1 ，2 和 4 处，我们需要使 num[i] < num[i+1] 。
下标 3 ，5 ，6 和 7 处，我们需要使 num[i] > num[i+1] 。
一些可能的 num 的值为 "245639871" ，"135749862" 和 "123849765" 。
"123549876" 是满足条件最小的数字。
注意，"123414321" 不是可行解因为数字 '1' 使用次数超过 1 次。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> pattern = "DDD"
<strong>输出:</strong> "4321"
<strong>解释:</strong>
一些可能的 num 的值为 "9876" ，"7321" 和 "8742" 。
"4321" 是满足条件最小的数字。
</pre>

#### 提示:
* `1 <= pattern.length <= 8`
* `pattern` 只包含字符 `'I'` 和 `'D'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut digits = (1..=pattern.len() + 1).rev().collect::<Vec<_>>();
        let mut stack = vec![];
        let mut num = vec![];

        for ch in pattern.chars().chain(std::iter::once('I')) {
            stack.push(digits.pop().unwrap() as u8 + b'0');

            while ch == 'I' && !stack.is_empty() {
                num.push(stack.pop().unwrap());
            }
        }

        String::from_utf8(num).unwrap()
    }
}
```
