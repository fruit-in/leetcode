impl Solution {
    pub fn minimum_buckets(hamsters: String) -> i32 {
        let hamsters = hamsters
            .replace("H.H", "B")
            .replace(".H", "B")
            .replace("H.", "B");

        if hamsters.contains('H') {
            -1
        } else {
            hamsters.chars().filter(|&c| c == 'B').count() as i32
        }
    }
}
