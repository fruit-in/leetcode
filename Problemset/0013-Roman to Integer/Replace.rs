impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.replace("IX", "IVV").replace("IV", "IIII");
        let s = s.replace("XC", "XLL").replace("XL", "XXXX");
        let s = s.replace("CM", "CDD").replace("CD", "CCCC");
 
        let mut n = 0;
        for c in s.chars() {
            match c {
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
