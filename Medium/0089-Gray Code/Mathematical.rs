impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut x = 1;
        let mut ret = vec![0];

        for _ in 0..n {
            let mut rev = ret.iter().rev().map(|&num| num + x).collect();
            ret.append(&mut rev);
            x *= 2;
        }

        ret
    }
}
