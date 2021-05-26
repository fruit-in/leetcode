# 1805. Number of Different Integers in a String
You are given a string `word` that consists of digits and lowercase English letters.

You will replace every non-digit character with a space. For example, `"a123bc34d8ef34"` will become `" 123  34 8  34"`. Notice that you are left with some integers that are separated by at least one space: `"123"`, `"34"`, `"8"`, and `"34"`.

Return *the number of **different** integers after performing the replacement operations on* `word`.

Two integers are considered different if their decimal representations **without any leading zeros** are different.

#### Example 1:
<pre>
<strong>Input:</strong> word = "a123bc34d8ef34"
<strong>Output:</strong> 3
<strong>Explanation:</strong> The three different integers are "123", "34", and "8". Notice that "34" is only counted once.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word = "leet1234code234"
<strong>Output:</strong> 2
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> word = "a1b01c001"
<strong>Output:</strong> 1
<strong>Explanation:</strong> The three integers "1", "01", and "001" all represent the same integer because
the leading zeros are ignored when comparing their decimal values.
</pre>

#### Constraints:
* `1 <= word.length <= 1000`
* `word` consists of digits and lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        word.split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_start_matches('0'))
            .collect::<HashSet<_>>()
            .len() as i32
    }
}
```
