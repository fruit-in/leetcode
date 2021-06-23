impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let num = num.to_string();
        let max = match num.chars().find(|&c| c < '9') {
            Some(c) => num.replace(c, "9"),
            None => num.clone(),
        };
        let min = if num.starts_with('1') {
            match num.chars().find(|&c| c > '1') {
                Some(c) => num.replace(c, "0"),
                None => num,
            }
        } else {
            num.replace(num.chars().next().unwrap(), "1")
        };

        max.parse::<i32>().unwrap() - min.parse::<i32>().unwrap()
    }
}
