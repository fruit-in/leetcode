impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut s1 = s1.into_bytes();
        let mut s2 = s2.into_bytes();
        s1.sort_unstable();
        s2.sort_unstable();
        let s1s2 = s1.iter().zip(s2.iter()).collect::<Vec<_>>();

        s1s2.iter().all(|tup| tup.0 >= tup.1) || s1s2.iter().all(|tup| tup.0 <= tup.1)
    }
}
