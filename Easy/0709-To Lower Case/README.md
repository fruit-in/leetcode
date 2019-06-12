# 709. To Lower Case
Implement function ToLowerCase() that has a string parameter str, and returns the same string in lowercase.

#### Example 1:
<pre>
<strong>Input:</strong> "Hello"
<strong>Output:</strong> "hello"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "here"
<strong>Output:</strong> "here"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "LOVELY"
<strong>Output:</strong> "lovely"
</pre>

## Solutions

### 1. Convert Every Chars (Rust)
```Rust
impl Solution {
    pub fn to_lower_case(str: String) -> String {
        let mut lower_case_str = String::new();
        for ch in str.chars() {
            lower_case_str.push(ch.to_ascii_lowercase());
        }
        lower_case_str
    }
}
```
