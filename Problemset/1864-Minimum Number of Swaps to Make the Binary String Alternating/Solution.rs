impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let zeros = s.chars().filter(|&c| c == '0').count();
        let ones = s.len() - zeros;
        let mut ret0 = i32::MAX;
        let mut ret1 = i32::MAX;

        if zeros == ones || zeros == ones + 1 {
            ret0 = s
                .bytes()
                .enumerate()
                .filter(|&(i, c)| (i % 2) as u8 + b'0' != c)
                .count() as i32
                / 2;
        }
        if zeros == ones || zeros + 1 == ones {
            ret1 = s
                .bytes()
                .enumerate()
                .filter(|&(i, c)| (i % 2) as u8 + b'0' == c)
                .count() as i32
                / 2;
        }

        if ret0 == i32::MAX && ret1 == i32::MAX {
            return -1;
        }

        ret0.min(ret1)
    }
}
