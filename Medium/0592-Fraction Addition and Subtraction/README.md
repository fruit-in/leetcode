# 592. Fraction Addition and Subtraction
Given a string representing an expression of fraction addition and subtraction, you need to return the calculation result in string format. The final result should be [irreducible fraction](https://en.wikipedia.org/wiki/Irreducible_fraction). If your final result is an integer, say ```2```, you need to change it to the format of fraction that has denominator ```1```. So in this case, ```2``` should be converted to ```2/1```.

#### Example 1:
<pre>
<strong>Input:</strong> "-1/2+1/2"
<strong>Output:</strong> "0/1"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "-1/2+1/2+1/3"
<strong>Output:</strong> "1/3"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "1/3-1/2"
<strong>Output:</strong> "-1/6"
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> "5/3+1/3"
<strong>Output:</strong> "2/1"
</pre>

#### Note:
1. The input string only contains ```'0'``` to ```'9'```, ```'/'```, ```'+'``` and ```'-'```. So does the output.
2. Each fraction (input and output) has format ```±numerator/denominator```. If the first input fraction or the output is positive, then ```'+'``` will be omitted.
3. The input only contains valid **irreducible fractions**, where the **numerator** and **denominator** of each fraction will always be in the range [1,10]. If the denominator is 1, it means this fraction is actually an integer in a fraction format defined above.
4. The number of given fractions will be in the range [1,10].
5. The numerator and denominator of the **final result** are guaranteed to be valid and in the range of 32-bit int.

## Solutions (Rust)

### 1. LCM
```Rust
impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let mut numerator = 0;
        let mut denominator = 1;
        let mut sum = 0;
        let mut is_numerator = true;
        let mut sign = 1;

        for ch in expression.bytes() {
            match ch {
                b'+' => {
                    sum += sign * numerator * (2520 / denominator);
                    numerator = 0;
                    is_numerator = true;
                    sign = 1;
                },
                b'-' => {
                    sum += sign * numerator * (2520 / denominator);
                    numerator = 0;
                    is_numerator = true;
                    sign = -1;
                },
                b'/' => {
                    is_numerator = false;
                    denominator = 0;
                },
                n if is_numerator => {
                    numerator *= 10;
                    numerator += (n - b'0') as i32;
                },
                n => {
                    denominator *= 10;
                    denominator += (n - b'0') as i32;
                }
            }
        }

        sum += sign * numerator * (2520 / denominator);
        denominator = 2520;

        for i in &[2, 2, 2, 3, 3, 5, 7] {
            if sum % i == 0 {
                sum /= i;
                denominator /= i;
            }
        }

        sum.to_string() + "/" + &denominator.to_string()
    }
}
```
