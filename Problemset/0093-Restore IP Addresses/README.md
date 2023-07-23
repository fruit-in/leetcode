# 93. Restore IP Addresses
Given a string containing only digits, restore it by returning all possible valid IP address combinations.

A valid IP address consists of exactly four integers (each integer is between 0 and 255) separated by single points.

#### Example:
<pre>
<strong>Input:</strong> "25525511135"
<strong>Output:</strong> ["255.255.11.135", "255.255.111.35"]
</pre>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        if s.len() < 4 || s.len() > 12 {
            return vec![];
        }

        let mut ret = vec![vec![s.as_str()]];

        for i in 0..3 {
            let mut temp = vec![];

            while let Some(v) = ret.pop() {
                let s = v[i];

                for j in 1..=3.min(s.len() + i - 3) {
                    let mut v = v[..i].to_vec();
                    let (n, remain) = s.split_at(j);

                    v.push(n);
                    v.push(remain);
                    temp.push(v);
                }
            }

            ret = temp;
        }

        ret.into_iter()
            .filter(|v| {
                v.iter()
                    .all(|&n| n == "0" || (!n.starts_with('0') && n.parse::<i32>().unwrap() < 256))
            })
            .map(|v| v.join("."))
            .collect()
    }
}
```
