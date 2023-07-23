# 468. Validate IP Address
Given a string `IP`, return `"IPv4"` if IP is a valid IPv4 address, `"IPv6"` if IP is a valid IPv6 address or `"Neither"` if IP is not a correct IP of any type.

**A valid IPv4** address is an IP in the form <code>"x<sub>1</sub>.x<sub>2</sub>.x<sub>3</sub>.x<sub>4</sub>"</code> where <code>0 <= x<sub>i</sub> <= 255</code> and <code>x<sub>i</sub></code> **cannot contain** leading zeros. For example, `"192.168.1.1"` and `"192.168.1.0"` are valid IPv4 addresses but `"192.168.01.1"`, while `"192.168.1.00"` and `"192.168@1.1"` are invalid IPv4 addresses.

**A valid IPv6** address is an IP in the form <code>"x<sub>1</sub>:x<sub>2</sub>:x<sub>3</sub>:x<sub>4</sub>:x<sub>5</sub>:x<sub>6</sub>:x<sub>7</sub>:x<sub>8</sub>"</code> where:
* <code>1 <= x<sub>i</sub>.length <= 4</code>
* <code>x<sub>i</sub></code> is a **hexadecimal string** which may contain digits, lower-case English letter (`'a'` to `'f'`) and upper-case English letters (`'A'` to `'F'`).
* Leading zeros are allowed in <code>x<sub>i</sub></code>.

For example, `"2001:0db8:85a3:0000:0000:8a2e:0370:7334"` and `"2001:db8:85a3:0:0:8A2E:0370:7334"` are valid IPv6 addresses, while `"2001:0db8:85a3::8A2E:037j:7334"` and `"02001:0db8:85a3:0000:0000:8a2e:0370:7334"` are invalid IPv6 addresses.

#### Example 1:
<pre>
<strong>Input:</strong> IP = "172.16.254.1"
<strong>Output:</strong> "IPv4"
<strong>Explanation:</strong> This is a valid IPv4 address, return "IPv4".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> IP = "2001:0db8:85a3:0:0:8A2E:0370:7334"
<strong>Output:</strong> "IPv6"
<strong>Explanation:</strong> This is a valid IPv6 address, return "IPv6".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> IP = "256.256.256.256"
<strong>Output:</strong> "Neither"
<strong>Explanation:</strong> This is neither a IPv4 address nor a IPv6 address.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> IP = "2001:0db8:85a3:0:0:8A2E:0370:7334:"
<strong>Output:</strong> "Neither"
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> IP = "1e1.4.5.6"
<strong>Output:</strong> "Neither"
</pre>

#### Constraints:
* `IP` consists only of English letters, digits and the characters `'.'` and `':'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn valid_ip_address(ip: String) -> String {
        let ipv4 = ip.split('.').collect::<Vec<_>>();
        let ipv6 = ip.split(':').collect::<Vec<_>>();

        if ipv4.len() == 4 && ipv4.into_iter().all(Self::valid_x_v4) {
            "IPv4"
        } else if ipv6.len() == 8 && ipv6.into_iter().all(Self::valid_x_v6) {
            "IPv6"
        } else {
            "Neither"
        }
        .to_string()
    }

    fn valid_x_v4(x: &str) -> bool {
        x == "0" || (!x.starts_with('0') && x.len() < 4 && x.parse::<i32>().unwrap_or(256) < 256)
    }

    fn valid_x_v6(x: &str) -> bool {
        x.len() > 0 && x.len() < 5 && x.chars().all(|c| c.is_digit(16))
    }
}
```
