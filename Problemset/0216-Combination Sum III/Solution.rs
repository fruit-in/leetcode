impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut x: i32 = (1 << k) - 1;
        let mut ret = vec![];

        while x < (1 << 9) {
            let comb = (1..=9)
                .filter(|&digit| (1 << (digit - 1)) & x != 0)
                .collect::<Vec<i32>>();

            if comb.iter().sum::<i32>() == n {
                ret.push(comb);
            }

            x += (x & -x) + (1 << ((x >> x.trailing_zeros()).trailing_ones() - 1)) - 1;
        }

        ret
    }
}
