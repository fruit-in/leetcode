# 1417. Reformat The String
Given alphanumeric string `s`. (**Alphanumeric string** is a string consisting of lowercase English letters and digits).

You have to find a permutation of the string where no letter is followed by another letter and no digit is followed by another digit. That is, no two adjacent characters have the same type.

Return *the reformatted string* or return **an empty string** if it is impossible to reformat the string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "a0b1c2"
<strong>Output:</strong> "0a1b2c"
<strong>Explanation:</strong> No two adjacent characters have the same type in "0a1b2c". "a0b1c2", "0a1b2c", "0c2a1b" are also valid permutations.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "leetcode"
<strong>Output:</strong> ""
<strong>Explanation:</strong> "leetcode" has only characters so we cannot separate them by digits.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "1229857369"
<strong>Output:</strong> ""
<strong>Explanation:</strong> "1229857369" has only digits so we cannot separate them by characters.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s = "covid2019"
<strong>Output:</strong> "c2o0v1i9d"
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> s = "ab123"
<strong>Output:</strong> "1a2b3"
</pre>

#### Constraints:
* `1 <= s.length <= 500`
* `s` consists of only lowercase English letters and/or digits.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn reformat(s: String) -> String {
        let (mut digits, mut letters): (Vec<u8>, Vec<u8>) =
            s.bytes().partition(|ch| ch.is_ascii_digit());
        let mut ret = vec![];

        let mut iterator = if digits.len() == letters.len() + 1 {
            ret.push(digits.pop().unwrap());
            letters.iter().zip(digits.iter())
        } else if digits.len() + 1 == letters.len() {
            ret.push(letters.pop().unwrap());
            digits.iter().zip(letters.iter())
        } else if digits.len() == letters.len() {
            digits.iter().zip(letters.iter())
        } else {
            [].iter().zip([].iter())
        };

        while let Some((&ch0, &ch1)) = iterator.next() {
            ret.push(ch0);
            ret.push(ch1);
        }

        String::from_utf8(ret).unwrap()
    }
}
```
