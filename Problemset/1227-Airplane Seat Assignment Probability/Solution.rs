impl Solution {
    pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
        match n {
            1 => 1.0,
            _ => 0.5,
        }
    }
}
