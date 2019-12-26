use std::collections::HashMap;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for n in deck {
            *map.entry(n).or_insert(0) += 1;
        }

        let mut x = *map.values().nth(0).unwrap();
        map.values().fold(x, |x, y| Self::gcd(x, *y)) > 1
    }

    pub fn gcd(mut x: i32, mut y: i32) -> i32 {
        while x % y != 0 {
            let tmp = x;
            x = y;
            y = tmp % y;
        }
        y
    }
}
