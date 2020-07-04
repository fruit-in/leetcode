# 93. 复原IP地址
给定一个只包含数字的字符串，复原它并返回所有可能的 IP 地址格式。

有效的 IP 地址正好由四个整数（每个整数位于 0 到 255 之间组成），整数之间用 `'.'` 分隔。

#### 示例:
<pre>
<strong>输入:</strong> "25525511135"
<strong>输出:</strong> ["255.255.11.135", "255.255.111.35"]
</pre>

## 题解 (Rust)

### 1. 题解
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
