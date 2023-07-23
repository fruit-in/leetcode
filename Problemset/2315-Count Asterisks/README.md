# 2315. Count Asterisks
You are given a string `s`, where every **two** consecutive vertical bars `'|'` are grouped into a **pair**. In other words, the 1st and 2nd `'|'` make a pair, the 3rd and 4th `'|'` make a pair, and so forth.

Return *the number of* `'*'` *in* `s`, ***excluding** the* `'*'` *between each pair of* `'|'`.

**Note** that each `'|'` will belong to **exactly** one pair.

#### Example 1:
<pre>
<strong>Input:</strong> s = "l|*e*et|c**o|*de|"
<strong>Output:</strong> 2
<strong>Explanation:</strong> The considered characters are underlined: "l|*e*et|c**o|*de|".
The characters between the first and second '|' are excluded from the answer.
Also, the characters between the third and fourth '|' are excluded from the answer.
There are 2 asterisks considered. Therefore, we return 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "iamprogrammer"
<strong>Output:</strong> 0
<strong>Explanation:</strong> In this example, there are no asterisks in s. Therefore, we return 0.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "yo|uar|e**|b|e***au|tifu|l"
<strong>Output:</strong> 5
<strong>Explanation:</strong> The considered characters are underlined: "yo|uar|e**|b|e***au|tifu|l". There are 5 asterisks considered. Therefore, we return 5.
</pre>

#### Constraints:
* `1 <= s.length <= 1000`
* `s` consists of lowercase English letters, vertical bars `'|'`, and asterisks `'*'`.
* `s` contains an **even** number of vertical bars `'|'`.

## Solutions (Rust)

### 1. Solution
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
