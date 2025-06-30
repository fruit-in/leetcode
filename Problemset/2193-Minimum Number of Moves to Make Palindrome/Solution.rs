impl Solution {
    pub fn min_moves_to_make_palindrome(s: String) -> i32 {
        let mut s = s.into_bytes();
        let n = s.len();
        let mut l = 0;
        let mut r = n - 1;
        let mut ret = 0;

        while l < r {
            for i in (l..=r).rev() {
                if i == l {
                    ret += (n / 2 - i) as i32;
                } else if s[i] == s[l] {
                    for j in i..r {
                        s.swap(j, j + 1);
                        ret += 1;
                    }

                    r -= 1;
                    break;
                }
            }

            l += 1;
        }

        ret
    }
}
