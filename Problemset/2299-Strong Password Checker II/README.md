# 2299. Strong Password Checker II
A password is said to be **strong** if it satisfies all the following criteria:
* It has at least `8` characters.
* It contains at least **one lowercase letter**.
* It contains at least **one uppercase letter**.
* It contains at least **one digit**.
* It contains at least **one special character**. The special characters are the characters in the following string: `"!@#$%^&*()-+"`.
* It does **not** contain `2` of the same character in adjacent positions (i.e., `"aab"` violates this condition, but `"aba"` does not).

Given a string `password`, return `true` *if it is a **strong** password*. Otherwise, return `false`.

#### Example 1:
<pre>
<strong>Input:</strong> password = "IloveLe3tcode!"
<strong>Output:</strong> true
<strong>Explanation:</strong> The password meets all the requirements. Therefore, we return true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> password = "Me+You--IsMyDream"
<strong>Output:</strong> false
<strong>Explanation:</strong> The password does not contain a digit and also contains 2 of the same character in adjacent positions. Therefore, we return false.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> password = "1aB!"
<strong>Output:</strong> false
<strong>Explanation:</strong> The password does not meet the length requirement. Therefore, we return false.
</pre>

#### Constraints:
* `1 <= password.length <= 100`
* `password` consists of letters, digits, and special characters: `"!@#$%^&*()-+"`.

## Solutions (Rust)

### 1. Solution
```Rust
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
```
