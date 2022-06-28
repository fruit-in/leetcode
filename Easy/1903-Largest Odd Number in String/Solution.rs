impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let mut num = num.into_bytes();

        while *num.last().unwrap_or(&1) % 2 == 0 {
            num.pop();
        }

        String::from_utf8(num).unwrap()
    }
}
