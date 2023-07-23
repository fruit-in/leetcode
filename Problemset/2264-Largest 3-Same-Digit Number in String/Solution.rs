impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let num = num.into_bytes();
        let goods = num.windows(3).filter(|n| n[0] == n[1] && n[1] == n[2]);
        let largest_good = goods.max().unwrap_or(&[]);

        String::from_utf8(largest_good.to_vec()).unwrap()
    }
}
