# 468. 验证IP地址
编写一个函数来验证输入的字符串是否是有效的 IPv4 或 IPv6 地址。
* 如果是有效的 IPv4 地址，返回 `"IPv4"` ；
* 如果是有效的 IPv6 地址，返回 `"IPv6"` ；
* 如果不是上述类型的 IP 地址，返回 `"Neither"` 。

**IPv4** 地址由十进制数和点来表示，每个地址包含 4 个十进制数，其范围为 0 - 255， 用(".")分割。比如，`172.16.254.1`；

同时，IPv4 地址内的数不会以 0 开头。比如，地址 `172.16.254.01` 是不合法的。

**IPv6** 地址由 8 组 16 进制的数字来表示，每组表示 16 比特。这些组数字通过 (":")分割。比如,  `2001:0db8:85a3:0000:0000:8a2e:0370:7334` 是一个有效的地址。而且，我们可以加入一些以 0 开头的数字，字母可以使用大写，也可以是小写。所以， `2001:db8:85a3:0:0:8A2E:0370:7334` 也是一个有效的 IPv6 address地址 (即，忽略 0 开头，忽略大小写)。

然而，我们不能因为某个组的值为 0，而使用一个空的组，以至于出现 (::) 的情况。 比如， `2001:0db8:85a3::8A2E:0370:7334` 是无效的 IPv6 地址。

同时，在 IPv6 地址中，多余的 0 也是不被允许的。比如， `02001:0db8:85a3:0000:0000:8a2e:0370:7334` 是无效的。

#### 示例 1:
<pre>
<strong>输入:</strong> IP = "172.16.254.1"
<strong>输出:</strong> "IPv4"
<strong>解释:</strong> 有效的 IPv4 地址，返回 "IPv4"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> IP = "2001:0db8:85a3:0:0:8A2E:0370:7334"
<strong>输出:</strong> "IPv6"
<strong>解释:</strong> 有效的 IPv6 地址，返回 "IPv6"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> IP = "256.256.256.256"
<strong>输出:</strong> "Neither"
<strong>解释:</strong> 既不是 IPv4 地址，又不是 IPv6 地址
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> IP = "2001:0db8:85a3:0:0:8A2E:0370:7334:"
<strong>输出:</strong> "Neither"
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> IP = "1e1.4.5.6"
<strong>输出:</strong> "Neither"
</pre>

#### 提示:
* `IP` 仅由英文字母，数字，字符 `'.'` 和 `':'` 组成。

## 题解 (Rust)

### 1. 题解
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
