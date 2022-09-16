# 2109. 向字符串添加空格
给你一个下标从 **0** 开始的字符串 `s` ，以及一个下标从 **0** 开始的整数数组 `spaces` 。

数组 `spaces` 描述原字符串中需要添加空格的下标。每个空格都应该插入到给定索引处的字符值 之前 。
* 例如，`s = "EnjoyYourCoffee"` 且 `spaces = [5, 9]` ，那么我们需要在 `'Y'` 和 `'C'` 之前添加空格，这两个字符分别位于下标 `5` 和下标 `9` 。因此，最终得到 `"Enjoy Your Coffee"` 。

请你添加空格，并返回修改后的字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "LeetcodeHelpsMeLearn", spaces = [8,13,15]
<strong>输出:</strong> "Leetcode Helps Me Learn"
<strong>解释:</strong>
下标 8、13 和 15 对应 "LeetcodeHelpsMeLearn" 中加粗斜体字符。
接着在这些字符前添加空格。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "icodeinpython", spaces = [1,5,7,9]
<strong>输出:</strong> "i code in py thon"
<strong>解释:</strong>
下标 1、5、7 和 9 对应 "icodeinpython" 中加粗斜体字符。
接着在这些字符前添加空格。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "spacing", spaces = [0,1,2,3,4,5,6]
<strong>输出:</strong> " s p a c i n g"
<strong>解释:</strong>
字符串的第一个字符前可以添加空格。
</pre>

#### 提示:
* <code>1 <= s.length <= 3 * 10<sup>5</sup></code>
* `s` 仅由大小写英文字母组成
* <code>1 <= spaces.length <= 3 * 10<sup>5</sup></code>
* `0 <= spaces[i] <= s.length - 1`
* `spaces` 中的所有值 **严格递增**

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let s = s.as_bytes();
        let mut prev = 0;
        let mut ret = vec![];

        for i in spaces {
            for j in prev..i as usize {
                ret.push(s[j]);
            }
            ret.push(b' ');
            prev = i as usize;
        }

        for j in prev..s.len() {
            ret.push(s[j]);
        }

        String::from_utf8(ret).unwrap()
    }
}
```
