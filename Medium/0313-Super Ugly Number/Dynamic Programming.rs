impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut uglys = vec![1];
        let mut points = vec![0; primes.len()];

        for _ in 1..n {
            let mut min_ugly = primes[0] * uglys[points[0]];
            for i in 1..points.len() {
                min_ugly = min_ugly.min(primes[i] * uglys[points[i]]);
            }

            uglys.push(min_ugly);

            for i in 0..points.len() {
                if primes[i] * uglys[points[i]] == min_ugly {
                    points[i] += 1;
                }
            }
        }

        uglys[n as usize - 1]
    }
}
