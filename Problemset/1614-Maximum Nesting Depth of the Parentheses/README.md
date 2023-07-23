# 1614. Maximum Nesting Depth of the Parentheses
A string is a **valid parentheses string** (denoted **VPS**) if it meets one of the following:
* It is an empty string `""`, or a single character not equal to `"("` or `")"`,
* It can be written as `AB` (`A` concatenated with `B`), where `A` and `B` are **VPS**'s, or
* It can be written as `(A)`, where `A` is a **VPS**.

We can similarly define the **nesting depth** `depth(S)` of any VPS `S` as follows:
* `depth("") = 0`
* `depth(C) = 0`, where `C` is a string with a single character not equal to `"("` or `")"`.
* `depth(A + B) = max(depth(A), depth(B))`, where `A` and `B` are **VPS**'s.
* `depth("(" + A + ")") = 1 + depth(A)`, where `A` is a **VPS**.

For example, `""`, `"()()"`, and `"()(()())"` are **VPS**'s (with nesting depths 0, 1, and 2), and `")("` and `"(()"` are not **VPS**'s.

Given a **VPS** represented as string `s`, return *the **nesting depth** of* `s`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "(1+(2*3)+((8)/4))+1"
<strong>Output:</strong> 3
<strong>Explanation:</strong> Digit 8 is inside of 3 nested parentheses in the string.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "(1)+((2))+(((3)))"
<strong>Output:</strong> 3
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "1+(2*3)/(2-1)"
<strong>Output:</strong> 1
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s = "1"
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `1 <= s.length <= 100`
* `s` consists of digits `0-9` and characters `'+'`, `'-'`, `'*'`, `'/'`, `'('`, and `')'`.
* It is guaranteed that parentheses expression `s` is a **VPS**.

## Solutions (Ruby)

### 1. Count
```Ruby
# @param {String} s
# @return {Integer}
def max_depth(s)
  left_count = 0
  ret = 0

  s.chars.each do |ch|
    if ch == '('
      left_count += 1
      ret = left_count if left_count > ret
    elsif ch == ')'
      left_count -= 1
    end
  end

  ret
end
```

## Solutions (Rust)

### 1. Count
```Rust
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut left_count = 0;
        let mut ret = 0;

        for ch in s.chars() {
            match ch {
                '(' => {
                    left_count += 1;
                    ret = ret.max(left_count);
                }
                ')' => left_count -= 1,
                _ => (),
            }
        }

        ret
    }
}
```
