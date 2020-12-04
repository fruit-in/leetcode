impl Solution {
    pub fn prime_palindrome(mut n: i32) -> i32 {
        loop {
            match n {
                999 | 99_999 | 9_999_999 => n = n * 10 + 11,
                n if Self::is_palindrome(n) && Self::is_prime(n) => return n,
                _ => n += 1,
            }
        }
    }

    fn is_palindrome(mut n: i32) -> bool {
        if n % 10 == 0 {
            return false;
        }

        let mut rev = 0;

        while n > rev {
            rev *= 10;
            rev += n % 10;
            n /= 10;
        }

        n == rev || n == rev / 10
    }

    fn is_prime(n: i32) -> bool {
        let mut i = 2;

        while i * i <= n && n % i != 0 {
            i += 1;
        }

        n > 1 && i * i > n
    }
}
