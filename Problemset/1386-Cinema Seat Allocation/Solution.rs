use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut seats_map = HashMap::new();
        let mut ret = 0;

        for seat in &reserved_seats {
            seats_map
                .entry(seat[0])
                .and_modify(|x| *x |= 1 << seat[1])
                .or_insert(1 << seat[1]);
        }

        for row in seats_map.values() {
            if row & 0x3fc == 0 {
                ret += 2;
            } else if row & 0x3c == 0 {
                ret += 1;
            } else if row & 0xf0 == 0 {
                ret += 1;
            } else if row & 0x3c0 == 0 {
                ret += 1;
            }
        }

        ret += (n - seats_map.len() as i32) * 2;

        ret
    }
}
