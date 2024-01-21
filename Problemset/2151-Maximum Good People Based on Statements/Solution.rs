impl Solution {
    pub fn maximum_good(statements: Vec<Vec<i32>>) -> i32 {
        let n = statements.len();
        let limit = 1 << n;

        for ret in (0..=n as i32).rev() {
            let mut x = (1 << ret) - 1;

            while x < limit {
                let mut flag = true;

                'outer: for i in 0..n {
                    if (x >> i) & 1 == 1 {
                        for j in 0..n {
                            if statements[i][j] < 2 && statements[i][j] != (x >> j) & 1 {
                                flag = false;
                                break 'outer;
                            }
                        }
                    }
                }

                if flag {
                    return ret;
                }

                let zeros = x.trailing_zeros();
                let ones = (x >> zeros).trailing_ones();
                x = (((x >> (zeros + ones)) + 1) << (zeros + ones)) + (1 << (ones - 1)) - 1;
            }
        }

        unreachable!()
    }
}
