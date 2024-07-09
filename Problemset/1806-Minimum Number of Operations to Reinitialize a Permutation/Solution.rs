impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        let n = n as usize;
        let mut perm = (0..n).collect::<Vec<_>>();

        for ret in 1..=n as i32 {
            let mut arr = vec![0; n];

            for i in 0..n {
                if i % 2 == 0 {
                    arr[i] = perm[i / 2];
                } else {
                    arr[i] = perm[n / 2 + (i - 1) / 2];
                }
            }

            perm = arr;

            if perm.iter().enumerate().all(|(i, &x)| i == x) {
                return ret;
            }
        }

        unreachable!()
    }
}
