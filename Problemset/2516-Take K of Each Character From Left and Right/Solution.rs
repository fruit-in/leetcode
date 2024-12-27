impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut count_a = 0;
        let mut count_b = 0;
        let mut count_c = 0;
        let mut i = 0;
        let mut ret: usize;

        while i < n && (count_a < k || count_b < k || count_c < k) {
            count_a += (s[i] == b'a') as i32;
            count_b += (s[i] == b'b') as i32;
            count_c += (s[i] == b'c') as i32;
            i += 1;
        }

        if count_a < k || count_b < k || count_c < k {
            return -1;
        }

        ret = i;

        for j in 1..n {
            count_a += (s[n - j] == b'a') as i32;
            count_b += (s[n - j] == b'b') as i32;
            count_c += (s[n - j] == b'c') as i32;

            while i > 0
                && count_a - (s[i - 1] == b'a') as i32 >= k
                && count_b - (s[i - 1] == b'b') as i32 >= k
                && count_c - (s[i - 1] == b'c') as i32 >= k
            {
                count_a -= (s[i - 1] == b'a') as i32;
                count_b -= (s[i - 1] == b'b') as i32;
                count_c -= (s[i - 1] == b'c') as i32;
                i -= 1;
            }

            if count_a >= k && count_b >= k && count_c >= k {
                ret = ret.min(i + j);
            }
        }

        ret as i32
    }
}
