impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let (mut m, mut n) = (m, n);
        let mut lo = 1;
        let mut hi = m * n;
        let mut flag = false;
        let mut count = 0;

        if m > n {
            (m, n) = (n, m);
        }

        while lo < hi {
            let mi = (lo + hi) / 2;
            flag = false;
            count = 0;

            for i in 1..=m {
                count += n.min(mi / i);
                flag |= mi % i == 0 && mi / i <= n;
            }

            if count < k {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }

        if flag {
            hi
        } else if count == k {
            (1..=m)
                .map(|i| n.min(hi / i + 1) * i)
                .filter(|&x| x > hi)
                .min()
                .unwrap()
        } else {
            (1..=m).map(|i| n.min(hi / i) * i).max().unwrap()
        }
    }
}
