impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut mask = 0;

        for c in s.chars() {
            if mask & (1 << (c as i32 - 97)) != 0 {
                return c;
            }

            mask |= 1 << (c as i32 - 97);
        }

        unimplemented!()
    }
}
