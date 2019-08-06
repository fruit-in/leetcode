# 1108. Defanging an IP Address
Given a valid (IPv4) IP <code>address</code>, return a defanged version of that IP address.

A *defanged IP address* replaces every period <code>"."</code> with <code>"[.]"</code>.

#### Example 1:
<pre>
<strong>Input:</strong> address = "1.1.1.1"
<strong>Output:</strong> "1[.]1[.]1[.]1"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> address = "255.100.50.0"
<strong>Output:</strong> "255[.]100[.]50[.]0"
</pre>

#### Constraints:
* The given <code>address</code> is a valid IPv4 address.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}
```
