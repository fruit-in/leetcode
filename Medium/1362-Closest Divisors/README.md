# 1362. Closest Divisors
Given an integer `num`, find the closest two integers in absolute difference whose product equals `num + 1` or `num + 2`.

Return the two integers in any order.

#### Example 1:
<pre>
<b>Input:</b> num = 8
<b>Output:</b> [3,3]
<b>Explanation:</b> For num + 1 = 9, the closest divisors are 3 & 3, for num + 2 = 10, the closest divisors are 2 & 5, hence 3 & 3 is chosen.
</pre>

#### Example 2:
<pre>
<b>Input:</b> num = 123
<b>Output:</b> [5,25]
</pre>

#### Example 3:
<pre>
<b>Input:</b> num = 999
<b>Output:</b> [40,25]
</pre>

#### Constraints:
* `1 <= num <= 10^9`

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer} num
# @return {Integer[]}
def closest_divisors(num)
    ret = [1, num + 1]

    for n in (num + 1)..(num + 2)
        for i in (Integer.sqrt(n)...1).step(-1)
            if n % i == 0 and n / i - i < ret[1] - ret[0]
                ret = [i, n / i]
                break
            end
        end
    end

    return ret
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let mut ret = vec![1, num + 1];

        for n in (num + 1)..(num + 3) {
            for i in (2..=((n as f64).sqrt().floor() as i32)).rev() {
                if n % i == 0 && n / i - i < ret[1] - ret[0] {
                    ret = vec![i, n / i];
                    break;
                }
            }
        }

        ret
    }
}
```
