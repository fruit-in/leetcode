impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let n = a.len();
        let mut prefix_i = -1;

        for i in 0..n / 2 {
            if a[i] == b[n - 1 - i] {
                prefix_i = i as i32;
            } else {
                break;
            }
        }
        for i in 0..n / 2 {
            if b[i] == a[n - 1 - i] {
                prefix_i = prefix_i.max(i as i32);
            } else {
                break;
            }
        }

        if prefix_i == n as i32 / 2 - 1 {
            return true;
        }

        for i in (0..=n / 2).rev() {
            if a[i] == a[n - 1 - i] && i as i32 - 1 <= prefix_i {
                return true;
            } else if a[i] != a[n - 1 - i] {
                break;
            }
        }
        for i in (0..=n / 2).rev() {
            if b[i] == b[n - 1 - i] && i as i32 - 1 <= prefix_i {
                return true;
            } else if b[i] != b[n - 1 - i] {
                break;
            }
        }

        false
    }
}
