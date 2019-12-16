impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        (2..=n).map(|num| num.to_string())
               .filter(|s| "2569".chars().any(|c| s.contains(c)))
               .filter(|s| s.chars().all(|c| "0182569".contains(c)))
               .count() as i32
    }
}
