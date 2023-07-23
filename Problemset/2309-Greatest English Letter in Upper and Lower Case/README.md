# 2309. Greatest English Letter in Upper and Lower Case
Given a string of English letters `s`, return *the **greatest** English letter which occurs as **both** a lowercase and uppercase letter in* `s`. The returned letter should be in **uppercase**. If no such letter exists, return *an empty string*.

An English letter `b` is **greater** than another letter `a` if `b` appears **after** `a` in the English alphabet.

#### Example 1:
<pre>
<strong>Input:</strong> s = "lEeTcOdE"
<strong>Output:</strong> "E"
<strong>Explanation:</strong>
The letter 'E' is the only letter to appear in both lower and upper case.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "arRAzFif"
<strong>Output:</strong> "R"
<strong>Explanation:</strong>
The letter 'R' is the greatest letter to appear in both lower and upper case.
Note that 'A' and 'F' also appear in both lower and upper case, but 'R' is greater than 'F' or 'A'.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "AbCdEfGhIjK"
<strong>Output:</strong> ""
<strong>Explanation:</strong>
There is no letter that appears in both lower and upper case.
</pre>

#### Constraints:
* `1 <= s.length <= 1000`
* `s` consists of lowercase and uppercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut occurs = [(false, false); 26];

        s.bytes().for_each(|c| match c {
            b'a'..=b'z' => occurs[(c - b'a') as usize].0 = true,
            _ => occurs[(c - b'A') as usize].1 = true,
        });

        match (0..26).rposition(|i| occurs[i] == (true, true)) {
            Some(i) => String::from_utf8(vec![i as u8 + b'A']).unwrap(),
            None => "".to_string(),
        }
    }
}
```
