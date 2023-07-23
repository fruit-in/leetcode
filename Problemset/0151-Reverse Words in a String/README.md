# 151. Reverse Words in a String
Given an input string, reverse the string word by word.

#### Example 1:
<pre>
<strong>Input:</strong> "the sky is blue"
<strong>Output:</strong> "blue is sky the"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "  hello world!  "
<strong>Output:</strong> "world! hello"
<strong>Explanation:</strong> Your reversed string should not contain leading or trailing spaces.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "a good   example"
<strong>Output:</strong> "example good a"
<strong>Explanation:</strong> You need to reduce multiple spaces between two words to a single space in the reversed string.
</pre>

#### Note:
* A word is defined as a sequence of non-space characters.
* Input string may contain leading or trailing spaces. However, your reversed string should not contain leading or trailing spaces.
* You need to reduce multiple spaces between two words to a single space in the reversed string.

#### Follow up:
For C programmers, try to solve it *in-place* in *O*(1) extra space.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
}
```
