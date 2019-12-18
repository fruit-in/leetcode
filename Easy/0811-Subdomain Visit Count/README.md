# 811. Subdomain Visit Count
A website domain like "discuss.leetcode.com" consists of various subdomains. At the top level, we have "com", at the next level, we have "leetcode.com", and at the lowest level, "discuss.leetcode.com". When we visit a domain like "discuss.leetcode.com", we will also visit the parent domains "leetcode.com" and "com" implicitly.

Now, call a "count-paired domain" to be a count (representing the number of visits this domain received), followed by a space, followed by the address. An example of a count-paired domain might be "9001 discuss.leetcode.com".

We are given a list ```cpdomains``` of count-paired domains. We would like a list of count-paired domains, (in the same format as the input, and in any order), that explicitly counts the number of visits to each subdomain.

<pre>
<strong>Example 1:</strong>
<strong>Input:</strong>
["9001 discuss.leetcode.com"]
<strong>Output:</strong>
["9001 discuss.leetcode.com", "9001 leetcode.com", "9001 com"]
<strong>Explanation:</strong>
We only have one website domain: "discuss.leetcode.com". As discussed above, the subdomain "leetcode.com" and "com" will also be visited. So they will all be visited 9001 times.
</pre>

<pre>
<strong>Example 2:</strong>
<strong>Input:</strong>
["900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"]
<strong>Output:</strong>
["901 mail.com","50 yahoo.com","900 google.mail.com","5 wiki.org","5 org","1 intel.mail.com","951 com"]
<strong>Explanation:</strong>
We will visit "google.mail.com" 900 times, "yahoo.com" 50 times, "intel.mail.com" once and "wiki.org" 5 times. For the subdomains, we will visit "mail.com" 900 + 1 = 901 times, "com" 900 + 50 + 1 = 951 times, and "org" 5 times.
</pre>

#### Note:
* The length of ```cpdomains``` will not exceed ```100```.
* The length of each domain name will not exceed ```100```.
* Each address will have either 1 or 2 "." characters.
* The input count in any count-paired domain will not exceed ```10000```.
* The answer output can be returned in any order.

## Solutions (Rust)

### 1. HashMap
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
