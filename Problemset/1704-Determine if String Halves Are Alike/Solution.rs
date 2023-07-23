impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        s.bytes()
            .enumerate()
            .filter(|(_, c)| b"aeiouAEIOU".contains(c))
            .map(|(i, _)| if i < s.len() / 2 { 1 } else { -1 })
            .sum::<i32>()
            == 0
    }
}
