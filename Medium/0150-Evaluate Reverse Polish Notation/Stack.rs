impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut nums: Vec<i32> = Vec::new();
        for token in tokens {
            if ["+", "-", "*", "/"].contains(&token.as_str()) {
                let temp1 = nums.pop().unwrap();
                let temp2 = nums.pop().unwrap();
                match token.as_str() {
                    "+" => nums.push(temp2 + temp1),
                    "-" => nums.push(temp2 - temp1),
                    "*" => nums.push(temp2 * temp1),
                    "/" => nums.push(temp2 / temp1),
                    _ => (),
                }
            } else {
                nums.push(token.parse().unwrap());
            }
        }
        nums.pop().unwrap()
    }
}
