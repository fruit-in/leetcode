impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut num1 = num1.bytes().map(|x| x - b'0').rev().collect::<Vec<_>>();
        let mut num2 = num2.bytes().map(|x| x - b'0').rev().collect::<Vec<_>>();
        let mut product = vec![0; num1.len() + num2.len()];

        for i in 0..num1.len() {
            for j in 0..num2.len() {
                product[i + j] += (num1[i] * num2[j]) as u32;
            }
        }

        for i in 0..product.len() {
            if product[i] > 9 {
                product[i + 1] += product[i] / 10;
                product[i] %= 10;
            }
        }

        while product.len() > 1 && *product.last().unwrap() == 0 {
            product.pop();
        }

        product
            .into_iter()
            .rev()
            .map(|x| char::from_digit(x, 10).unwrap())
            .collect()
    }
}
