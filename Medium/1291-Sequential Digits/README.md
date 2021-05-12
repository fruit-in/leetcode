# 1291. Sequential Digits
An integer has *sequential digits* if and only if each digit in the number is one more than the previous digit.

Return a **sorted** list of all the integers in the range `[low, high]` inclusive that have sequential digits.

#### Example 1:
<pre>
<strong>Input:</strong> low = 100, high = 300
<strong>Output:</strong> [123,234]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> low = 1000, high = 13000
<strong>Output:</strong> [1234,2345,3456,4567,5678,6789,12345]
</pre>

#### Constraints:
* `10 <= low <= high <= 10^9`

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer} low
# @param {Integer} high
# @return {Integer[]}
def sequential_digits(low, high)
  ret = []

  (1..8).each do |x|
    while x <= high && x % 10 != 0
      ret.push(x) if x >= low
      x = x * 10 + x % 10 + 1
    end
  end

  ret.sort
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ret = vec![];

        for i in 1..9 {
            let mut x = i;
            while x <= high && x % 10 != 0 {
                if x >= low {
                    ret.push(x);
                }
                x = x * 10 + x % 10 + 1;
            }
        }

        ret.sort_unstable();

        ret
    }
}
```
