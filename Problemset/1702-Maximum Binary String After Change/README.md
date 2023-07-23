# 1702. Maximum Binary String After Change
You are given a binary string `binary` consisting of only `0`'s or `1`'s. You can apply each of the following operations any number of times:
* Operation 1: If the number contains the substring `"00"`, you can replace it with `"10"`.
    * For example, `"00010" -> "10010"`
* Operation 2: If the number contains the substring `"10"`, you can replace it with `"01"`.
    * For example, `"00010" -> "00001"`

*Return the **maximum binary string** you can obtain after any number of operations. Binary string* `x` *is greater than binary string* `y` *if* `x`*'s decimal representation is greater than* `y`*'s decimal representation*.

#### Example 1:
<pre>
<strong>Input:</strong> binary = "000110"
<strong>Output:</strong> "111011"
<strong>Explanation:</strong> A valid transformation sequence can be:
"000110" -> "000101"
"000101" -> "100101"
"100101" -> "110101"
"110101" -> "110011"
"110011" -> "111011"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> binary = "01"
<strong>Output:</strong> "01"
<strong>Explanation:</strong> "01" cannot be transformed any further.
</pre>

#### Constraints:
* <code>1 <= binary.length <= 10<sup>5</sup></code>
* `binary` consist of `'0'` and `'1'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let count0 = binary.bytes().filter(|&b| b == b'0').count();
        let count1 = binary.bytes().take_while(|&b| b == b'1').count();
        let mut binary = binary.into_bytes();

        if count0 > 1 {
            binary = vec![b'1'; binary.len()];
            binary[count0 + count1 - 1] = b'0';
        }

        String::from_utf8(binary).unwrap()
    }
}
```
