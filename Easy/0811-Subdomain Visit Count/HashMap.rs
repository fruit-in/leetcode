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
