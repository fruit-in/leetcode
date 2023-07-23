impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut n = 0;
        if s.contains("IV") { n -= 2; }
        if s.contains("IX") { n -= 2; }
        if s.contains("XL") { n -= 20; }
        if s.contains("XC") { n -= 20; }
        if s.contains("CD") { n -= 200; }
        if s.contains("CM") { n -= 200; }
 
        for ch in s.chars() {
            match ch {
                'I' => n += 1,
                'V' => n += 5,
                'X' => n += 10,
                'L' => n += 50,
                'C' => n += 100,
                'D' => n += 500,
                'M' => n += 1000,
                _ => (),
            };
        }
        n
    }
}
