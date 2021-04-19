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
