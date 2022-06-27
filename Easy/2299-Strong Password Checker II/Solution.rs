impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        password.len() >= 8
            && password.chars().any(|c| c.is_lowercase())
            && password.chars().any(|c| c.is_uppercase())
            && password.chars().any(|c| c.is_digit(10))
            && password.chars().any(|c| "!@#$%^&*()-+".contains(c))
            && password
                .chars()
                .zip(password.chars().skip(1))
                .all(|(c1, c2)| c1 != c2)
    }
}
