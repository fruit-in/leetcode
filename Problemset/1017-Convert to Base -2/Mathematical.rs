impl Solution {
    pub fn base_neg2(n: i32) -> String {
        if n == 0 {
            return "0".to_string();
        }

        let mut n = n;
        let mut ret = Vec::new();

        while n != 0 {
            let x = n % 4;
            match x {
                0 => ret.push("00"),
                1 if n != 1 => ret.push("01"),
                2 => ret.push("10"),
                3 => ret.push("11"),
                _ => ret.push("1"),
            }
            n = n / 4 + x / 2;
        }

        ret.reverse();
        ret.concat()
    }
}
