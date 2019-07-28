impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        if num == 0 {
            false
        } else if num == 1 {
            true
        } else if num % 2 == 0 {
            Self::is_ugly(num / 2)
        } else if num % 3 == 0 {
            Self::is_ugly(num / 3)
        } else if num % 5 == 0 {
            Self::is_ugly(num / 5)
        } else {
            false
        }
    }
}
