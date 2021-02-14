# 1678. Goal Parser Interpretation
You own a **Goal Parser** that can interpret a string `command`. The `command` consists of an alphabet of `"G"`, `"()"` and/or `"(al)"` in some order. The Goal Parser will interpret `"G"` as the string `"G"`, `"()"` as the string `"o"`, and `"(al)"` as the string `"al"`. The interpreted strings are then concatenated in the original order.

Given the string `command`, return *the **Goal Parser**'s interpretation of* `command`.

#### Example 1:
<pre>
<strong>Input:</strong> command = "G()(al)"
<strong>Output:</strong> "Goal"
<strong>Explanation:</strong> The Goal Parser interprets the command as follows:
G -> G
() -> o
(al) -> al
The final concatenated result is "Goal".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> command = "G()()()()(al)"
<strong>Output:</strong> "Gooooal"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> command = "(al)G(al)()()G"
<strong>Output:</strong> "alGalooG"
</pre>

#### Constraints:
* `1 <= command.length <= 100`
* `command` consists of `"G"`, `"()"`, and/or `"(al)"` in some order.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn interpret(command: String) -> String {
        (command + " ")
            .as_bytes()
            .windows(2)
            .map(|w| match (w[0], w[1]) {
                (b'G', _) => "G",
                (b'(', b')') => "o",
                (b'(', b'a') => "al",
                _ => "",
            })
            .collect()
    }
}
```
