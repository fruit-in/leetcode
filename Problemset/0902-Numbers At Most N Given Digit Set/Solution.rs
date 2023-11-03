impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let digits = digits
            .iter()
            .map(|d| d.parse::<u8>().unwrap() + b'0')
            .collect::<Vec<_>>();
        let digitsn = n.to_string().into_bytes();
        let q = digits.len();
        let n = digitsn.len();
        let mut ret = n;

        if q > 1 {
            ret = q * (q.pow(n as u32) - 1) / (q - 1);

            for i in 0..n {
                match digits.binary_search(&digitsn[i]) {
                    Ok(j) => ret -= q.pow((n - i - 1) as u32) * (q - j - 1),
                    Err(j) => {
                        ret -= q.pow((n - i - 1) as u32) * (q - j);
                        break;
                    }
                }
            }
        } else if vec![digits[0]; n] > digitsn {
            ret -= 1;
        }

        ret as i32
    }
}
