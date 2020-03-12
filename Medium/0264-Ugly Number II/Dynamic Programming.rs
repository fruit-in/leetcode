impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut uglys = vec![1];
        let mut p2 = 0;
        let mut p3 = 0;
        let mut p5 = 0;

        for _ in 1..n {
            let min_ugly = (2 * uglys[p2]).min(3 * uglys[p3]).min(5 * uglys[p5]);
            uglys.push(min_ugly);

            if min_ugly == 2 * uglys[p2] {
                p2 += 1;
            }
            if min_ugly == 3 * uglys[p3] {
                p3 += 1;
            }
            if min_ugly == 5 * uglys[p5] {
                p5 += 1;
            }
        }

        uglys[n as usize - 1]
    }
}
