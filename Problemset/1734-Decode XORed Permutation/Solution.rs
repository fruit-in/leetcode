impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let n = encoded.len() + 1;
        let m = n / 2;
        let mut perm = vec![0; n];
        perm[m] = m as i32 + 1;

        for i in 0..m {
            perm[m] ^= ((i + 1) ^ (n - i)) as i32;
            if i % 2 == 0 {
                perm[m] ^= encoded[i] ^ encoded[n - 2 - i];
            }
        }

        for i in 1..=m {
            perm[m - i] = encoded[m - i] ^ perm[m - i + 1];
            perm[m + i] = encoded[m + i - 1] ^ perm[m + i - 1];
        }

        perm
    }
}
