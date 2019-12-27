# 811. 子域名访问计数
一个网站域名，如"discuss.leetcode.com"，包含了多个子域名。作为顶级域名，常用的有"com"，下一级则有"leetcode.com"，最低的一级为"discuss.leetcode.com"。当我们访问域名"discuss.leetcode.com"时，也同时访问了其父域名"leetcode.com"以及顶级域名 "com"。

给定一个带访问次数和域名的组合，要求分别计算每个域名被访问的次数。其格式为访问次数+空格+地址，例如："9001 discuss.leetcode.com"。

接下来会给出一组访问次数和域名组合的列表```cpdomains``` 。要求解析出所有域名的访问次数，输出格式和输入格式相同，不限定先后顺序。

<pre>
<strong>示例 1:</strong>
<strong>输入:</strong>
["9001 discuss.leetcode.com"]
<strong>输出:</strong>
["9001 discuss.leetcode.com", "9001 leetcode.com", "9001 com"]
<strong>说明:</strong>
例子中仅包含一个网站域名："discuss.leetcode.com"。按照前文假设，子域名"leetcode.com"和"com"都会被访问，所以它们都被访问了9001次。
</pre>

<pre>
<strong>示例 2:</strong>
<strong>输入:</strong>
["900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"]
<strong>输出:</strong>
["901 mail.com","50 yahoo.com","900 google.mail.com","5 wiki.org","5 org","1 intel.mail.com","951 com"]
<strong>说明:</strong>
按照假设，会访问"google.mail.com" 900次，"yahoo.com" 50次，"intel.mail.com" 1次，"wiki.org" 5次。
而对于父域名，会访问"mail.com" 900+1 = 901次，"com" 900 + 50 + 1 = 951次，和 "org" 5 次。
</pre>

#### 注意事项:
* ```cpdomains``` 的长度小于 ```100```。
* 每个域名的长度小于```100```。
* 每个域名地址包含一个或两个"."符号。
* 输入中任意一个域名的访问次数都小于```10000```。

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut domain_cp = HashMap::new();

        for cpdomain in cpdomains {
            let cp_domain = cpdomain.split(' ').collect::<Vec<_>>();
            let domains = cp_domain[1].split('.').collect::<Vec<_>>();

            let mut prev = String::new();
            for i in (0..domains.len()).rev() {
                prev.insert_str(0, domains[i]);
                let cnt = domain_cp.entry(prev.clone()).or_insert(0);
                *cnt += cp_domain[0].parse::<i32>().unwrap();
                prev.insert(0, '.');
            }
        }

        domain_cp.drain()
                 .map(|(domain, cp)| cp.to_string() + " " + &domain)
                 .collect()
    }
}
```
